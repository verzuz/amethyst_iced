use amethyst::ecs::{Read, SystemData, World, Write};
use amethyst::renderer::{
    rendy::{
        command::{QueueId, RenderPassEncoder},
        factory::Factory,
        graph::{
            render::{PrepareResult, RenderGroup, RenderGroupDesc},
            GraphContext, NodeBuffer, NodeImage,
        },
        hal::{self},
    },
    types::Backend,
};
use glsl_layout::AsStd140;

use crate::pipelines::{ImagePipeline, TextPipeline, TrianglePipeline};
use crate::systems::TextVertexContainer;
use crate::{vertex::TriangleVertex};
use iced_graphics::{Primitive, Layer};

#[derive(Default, Debug)]
pub struct IcedPassDesc;

impl<B: Backend> RenderGroupDesc<B, World> for IcedPassDesc {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _world: &World,
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

        Ok(Box::new(IcedPass {
            triangle_pipeline,
            image_pipeline,
            text_pipeline,
            prev_hash_layout: vec![0, 0, 0, 0, 0],
        }))
    }
}

#[derive(Debug)]
pub struct IcedPass<B: Backend> {
    pub triangle_pipeline: TrianglePipeline<B>,
    pub image_pipeline: ImagePipeline<B>,
    pub text_pipeline: TextPipeline<B>,
    pub prev_hash_layout: Vec<u64>,
}

impl<B: Backend> RenderGroup<B, World> for IcedPass<B> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {
        let mut iced_primitives = Write::<'_, Primitive>::fetch(world);

        self.image_pipeline.reset(factory, index);
        self.text_pipeline.reset(factory, index, world);

        self.triangle_pipeline.vertices = vec![];
        self.triangle_pipeline.uniforms.write(
            factory,
            index,
            self.triangle_pipeline.transform.std140(),
        );

        {
            let iced_primitives = *iced_primitives;

            let viewport_size = viewport.physical_size();
            let scale_factor = viewport.scale_factor() as f32;
            let projection = viewport.projection();
    
            let mut layers = Layer::generate(iced_primitives, viewport);
//            layers.push(Layer::overlay(overlay_text, viewport));
    
            for layer in layers {
                self.flush(
                    gl,
                    scale_factor,
                    projection,
                    &layer,
                    viewport_size.height,
                );
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

        let text_vertex_container = Read::<'_, TextVertexContainer>::fetch(world);
        self.text_pipeline.vertex.write(
            factory,
            index,
            text_vertex_container.0.len() as u64,
            Some(&(text_vertex_container.0)),
        );

        self.text_pipeline.textures.maintain(factory, world);
        self.image_pipeline.textures.maintain(factory, world);

        /*
        if let Some(hash) = self.prev_hash_layout.get(index) {
            if iced_primitives.1 == *hash { return PrepareResult::DrawReuse; }
        }
        self.prev_hash_layout[index] = iced_primitives.1;
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
        self.triangle_pipeline.draw(&mut encoder, index);
        self.image_pipeline.draw(&mut encoder, index);
        self.text_pipeline.draw(&mut encoder, index, aux);
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        self.triangle_pipeline.dispose(factory);
        self.image_pipeline.dispose(factory);
        self.text_pipeline.dispose(factory);
    }


}
