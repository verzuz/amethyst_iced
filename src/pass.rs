use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::ecs::{Read, SystemData, World, Write, WriteExpect};
use amethyst::renderer::{
    rendy::{
        command::{QueueId, RenderPassEncoder},
        factory::{Factory, ImageState},
        graph::{
            render::{PrepareResult, RenderGroup, RenderGroupDesc},
            GraphContext, NodeBuffer, NodeImage,
        },
        hal::{self, pso::Rect},
        texture::{pixel::R8Unorm, TextureBuilder},
    },
    types::Backend,
    Texture,
};
use amethyst::ui::{FontAsset, TtfFormat};
use glsl_layout::AsStd140;

use crate::pipelines::{ImagePipeline, TextPipeline, TrianglePipeline};
use crate::resources::FontCache;
use crate::systems::{GlyphAtlas};
use crate::vertex::{TextVertex, TriangleVertex};
use crate::IcedGlyphBrush;
use glyph_brush::{BrushAction, BrushError,rusttype::Scale, FontId, HorizontalAlign, Layout, Section, VerticalAlign};
use iced_graphics::{Layer, Primitive, Rectangle, Size, Viewport};
use iced_native::{Font, HorizontalAlignment, VerticalAlignment};

#[derive(Default, Debug)]
pub struct IcedPassDesc;

impl<B: Backend> RenderGroupDesc<B, World> for IcedPassDesc {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        world: &World,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: hal::pass::Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        let triangle_pipeline = TrianglePipeline::create_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
        )?;

        let image_pipeline = ImagePipeline::create_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
        )?;

        let text_pipeline =
            TextPipeline::create_pipeline(factory, subpass, framebuffer_width, framebuffer_height)?;

        let mut font_cache = Write::<'_, FontCache>::fetch(world);
        let font_storage = world.fetch::<AssetStorage<FontAsset>>();
        let loader = world.fetch::<Loader>();
        let font_handle = loader.load("font/OpenSans-Regular.ttf", TtfFormat, (), &font_storage);
        font_cache.insert("opensans_regular".to_string(), font_handle.clone());

        Ok(Box::new(IcedPass {
            triangle_pipeline,
            image_pipeline,
            text_pipeline,
            prev_hash_layout: vec![0, 0, 0, 0, 0],
            layer_infos: Vec::new(),
            default_font: font_handle,
            text_vertices : Vec::new(),
        }))
    }
}

#[derive(Debug)]
pub struct IcedPass<B: Backend> {
    pub triangle_pipeline: TrianglePipeline<B>,
    pub image_pipeline: ImagePipeline<B>,
    pub text_pipeline: TextPipeline<B>,
    pub prev_hash_layout: Vec<u64>,
    layer_infos: Vec<LayerInfo>,
    default_font: Handle<FontAsset>,
    text_vertices : Vec<TextVertex>,
}

#[derive(Debug)]
struct LayerInfo {
    text_start: u32,
    text_count: u32,
    quad_start: u32,
    quad_count: u32,
    triangle_start: u32,
    triangle_count: u32,
    bounds: Rect,
}

impl Default for LayerInfo {
    fn default() -> Self {
        LayerInfo {
            text_start: 0,
            text_count: 0,
            quad_start: 0,
            quad_count: 0,
            triangle_start: 0,
            triangle_count: 0,
            bounds: Rect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
        }
    }
}

impl<B: Backend> IcedPass<B> {
    fn compute_renderdata(&mut self, layer: Layer, world: &World) -> LayerInfo {
        let mut layerinfo = LayerInfo::default();
        layerinfo.quad_start = self.triangle_pipeline.vertices.len() as u32;
        for quad in &layer.quads {
            let rect = Rectangle::new(quad.position.into(), quad.size.into());

            self.triangle_pipeline.vertices.extend_from_slice(&[
                TriangleVertex {
                    position: [rect.x, rect.y].into(),
                    color: quad.color.into(),
                },
                TriangleVertex {
                    position: [rect.x + rect.width, rect.y].into(),
                    color: quad.color.into(),
                },
                TriangleVertex {
                    position: [rect.x + rect.width, rect.y + rect.height].into(),
                    color: quad.color.into(),
                },
                TriangleVertex {
                    position: [rect.x, rect.y].into(),
                    color: quad.color.into(),
                },
                TriangleVertex {
                    position: [rect.x, rect.y + rect.height].into(),
                    color: quad.color.into(),
                },
                TriangleVertex {
                    position: [rect.x + rect.width, rect.y + rect.height].into(),
                    color: quad.color.into(),
                },
            ]);
        }

        layerinfo.quad_count = layer.quads.len() as u32 * 6;

        let mut font_cache = Write::<'_, FontCache>::fetch(world);
        let mut iced_glyph_brush = WriteExpect::<'_, IcedGlyphBrush>::fetch(world);

        layerinfo.text_start = 0;
        layerinfo.text_count = layer.text.len() as u32;

        for text in &layer.text {
            let font_id = match text.font {
                Font::Default => FontId::default(),
                Font::External { name, .. } => font_cache.get_id(name).cloned().unwrap_or_default(),
            };

            iced_glyph_brush.queue(Section {
                font_id: font_id,
                text: &text.content,
                color: text.color,
                scale: Scale::uniform(text.size as f32),
                bounds: (text.bounds.width, text.bounds.height),
                screen_position: (text.bounds.x, text.bounds.y),
                layout: Layout::default()
                    .h_align(match text.horizontal_alignment {
                        HorizontalAlignment::Left => HorizontalAlign::Left,
                        HorizontalAlignment::Center => HorizontalAlign::Center,
                        HorizontalAlignment::Right => HorizontalAlign::Right,
                    })
                    .v_align(match text.vertical_alignment {
                        VerticalAlignment::Top => VerticalAlign::Top,
                        VerticalAlignment::Center => VerticalAlign::Center,
                        VerticalAlignment::Bottom => VerticalAlign::Bottom,
                    }),
                ..Section::default()
            });
        }

        layerinfo.bounds = Rect {
            x: layer.bounds.x as i16,
            y: layer.bounds.y as i16,
            w: layer.bounds.width as i16,
            h: layer.bounds.height as i16,
        };
        layerinfo
    }
}

impl<B: Backend> RenderGroup<B, World> for IcedPass<B> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {

        let iced_primitives = Write::<'_, Primitive>::fetch(world);

        let action = {
            let mut glyph_brush = WriteExpect::<'_, IcedGlyphBrush>::fetch(world);
            let asset_textures = Write::<'_, AssetStorage<Texture>>::fetch(world);
                let glyph_atlas = Write::<'_, GlyphAtlas>::fetch(world);
            let glyph_tex = asset_textures
                .get(&glyph_atlas.0.as_ref().unwrap())
                .and_then(B::unwrap_texture)
                .unwrap();
            glyph_brush.process_queued(
                |rect, data| unsafe {
                    factory
                        .upload_image(
                            glyph_tex.image().clone(),
                            rect.width(),
                            rect.height(),
                            hal::image::SubresourceLayers {
                                aspects: hal::format::Aspects::COLOR,
                                level: 0,
                                layers: 0..1,
                            },
                            hal::image::Offset {
                                x: rect.min.x as _,
                                y: rect.min.y as _,
                                z: 0,
                            },
                            hal::image::Extent {
                                width: rect.width(),
                                height: rect.height(),
                                depth: 1,
                            },
                            data,
                            ImageState {
                                queue,
                                stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                                access: hal::image::Access::SHADER_READ,
                                layout: hal::image::Layout::General,
                            },
                            ImageState {
                                queue,
                                stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                                access: hal::image::Access::SHADER_READ,
                                layout: hal::image::Layout::General,
                            },
                        )
                        .unwrap();
                },
                |glyph| {
                    // TODO: dont display glyph if out of screen bounds

                    let uvs = glyph.tex_coords;
                    //let pos = glyph.pixel_coords;
                    let pos = glyph.pixel_coords;
                    let color: [f32; 4] = glyph.color;

                    (
                        glyph.z.to_bits(),
                        vec![
                            TextVertex {
                                position: [pos.min.x as f32, pos.min.y as f32].into(),
                                uv: [uvs.min.x, uvs.min.y].into(),
                                color: color.into(),
                            },
                            TextVertex {
                                position: [pos.max.x as f32, pos.min.y as f32].into(),
                                uv: [uvs.max.x, uvs.min.y].into(),
                                color: color.into(),
                            },
                            TextVertex {
                                position: [pos.max.x as f32, pos.max.y as f32].into(),
                                uv: [uvs.max.x, uvs.max.y].into(),
                                color: color.into(),
                            },
                            TextVertex {
                                position: [pos.min.x as f32, pos.min.y as f32].into(),
                                uv: [uvs.min.x, uvs.min.y].into(),
                                color: color.into(),
                            },
                            TextVertex {
                                position: [pos.min.x as f32, pos.max.y as f32].into(),
                                uv: [uvs.min.x, uvs.max.y].into(),
                                color: color.into(),
                            },
                            TextVertex {
                                position: [pos.max.x as f32, pos.max.y as f32].into(),
                                uv: [uvs.max.x, uvs.max.y].into(),
                                color: color.into(),
                            },
                        ],
                    )
                },
            )
        };


        self.image_pipeline.reset(factory, index);

        self.triangle_pipeline.vertices = vec![];
        self.triangle_pipeline.uniforms.write(
            factory,
            index,
            self.triangle_pipeline.transform.std140(),
        );

        self.layer_infos.clear();

        {
            let iced_primitives = &*iced_primitives;

            let viewport = Viewport::with_physical_size(Size::new(1500, 1500), 1.5);

            //LayerInfo? trianglecount, ...
            let layers = Layer::generate(iced_primitives, &viewport);

            for layer in layers {
                let info = self.compute_renderdata(layer, &world);
                self.layer_infos.push(info);
            }
        }

        self.triangle_pipeline.vertex.write(
            factory,
            index,
            self.triangle_pipeline.vertices.len() as u64,
            Some(
                self.triangle_pipeline
                    .vertices
                    .clone()
                    .into_iter()
                    .collect::<Box<[TriangleVertex]>>(),
            ),
        );
        self.image_pipeline.vertex.write(
            factory,
            index,
            self.image_pipeline.batches.count() as u64,
            Some(self.image_pipeline.batches.data()),
        );

        self.text_pipeline.reset(factory, index, world);
        match action {
            Ok(BrushAction::Draw(vertices)) => {
                let vertices : Vec<TextVertex> = vertices
                .into_iter()
                .flat_map(|(_id, verts)| verts.into_iter())
                .collect();
                self.text_vertices = vertices;
            }
            Err(BrushError::TextureTooSmall { suggested }) => {
                self.text_vertices = Vec::new();
                println!("brusherror. Suggest {:?}", suggested);
            }
            _ => {}
        }
        self.text_pipeline.vertex.write(
            factory,
            index,
            self.text_vertices.len() as u64,
            Some(&self.text_vertices),
        );


        self.text_pipeline.textures.maintain(factory, world);
        self.image_pipeline.textures.maintain(factory, world);

        /*
        if let Some(hash) = self.prev_hash_layout.get(index) {
            if iced_primitives == *hash { return PrepareResult::DrawReuse; }
        }
        self.prev_hash_layout[index] = iced_primitives;
        */
        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        aux: &World,
    ) {
        for layerinfo in &self.layer_infos {
            self.triangle_pipeline.draw(
                &mut encoder,
                index,
                layerinfo.quad_start,
                layerinfo.quad_count,
                layerinfo.bounds,
            );
        }
        self.text_pipeline.draw(
            &mut encoder,
            index,
            aux,
            0, //TODO start
            self.text_vertices.len() as u32, //TODO count
        );

        self.image_pipeline.draw(&mut encoder, index);
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        self.triangle_pipeline.dispose(factory);
        self.image_pipeline.dispose(factory);
        self.text_pipeline.dispose(factory);
    }
}
