use amethyst::assets::{AssetStorage, Handle};
use amethyst::ecs::{Read, ReadExpect, System, SystemData, World, Write, WriteExpect};
use amethyst::renderer::{
    rendy::{
        command::QueueId,
        factory::{Factory, ImageState},
        hal,
        texture::{pixel::R8Unorm, TextureBuilder},
    },
    types::Backend,
    SpriteSheet, Texture,
};

use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::window::ScreenDimensions;
use amethyst::winit::{
    ElementState, Event as WinitEvent, MouseButton, WindowEvent as WinitWindowEvent,
};
use iced_graphics::Primitive;
use iced_native::{Cache, Size, UserInterface};

use crate::backend::IcedRenderer;
use crate::resources::{FontCache, ImageCache};
use crate::sandbox::{Sandbox, SandboxContainer};
use crate::vertex::TextInfo;
use crate::IcedGlyphBrush;

use std::hash::Hasher;
#[derive(Default)]
pub struct GlyphAtlas(pub Option<Handle<Texture>>);

#[derive(Default)]
pub struct TextVertexContainer(pub Vec<TextInfo>);

pub(crate) struct IcedDrawSystem<S: Sandbox, B: Backend> {
    _sandbox: std::marker::PhantomData<S>,
    _backend: std::marker::PhantomData<B>,
    winit_reader_id: Option<ReaderId<WinitEvent>>,
    cache: Option<Cache>,
}

impl<S: Sandbox, B: Backend> Default for IcedDrawSystem<S, B> {
    fn default() -> Self {
        IcedDrawSystem {
            _sandbox: std::marker::PhantomData,
            _backend: std::marker::PhantomData,
            winit_reader_id: None,
            cache: Some(Cache::default()),
        }
    }
}

impl<'a, S: Sandbox, B: Backend> System<'a> for IcedDrawSystem<S, B> {
    type SystemData = (
        Read<'a, EventChannel<WinitEvent>>,
        Write<'a, EventChannel<<S as Sandbox>::UIMessage>>,
        Option<Write<'a, SandboxContainer<S>>>,
        Read<'a, AssetStorage<SpriteSheet>>,
        WriteExpect<'a, IcedGlyphBrush>,
        Read<'a, FontCache>,
        ReadExpect<'a, ScreenDimensions>,
        Write<'a, Primitive>,
        Write<'a, AssetStorage<Texture>>,
        WriteExpect<'a, Factory<B>>,
        Option<Read<'a, QueueId>>,
        Write<'a, GlyphAtlas>,
        Write<'a, ImageCache>,
    );

    fn run(
        &mut self,
        (
            winit_events,
            mut ui_messages,
            sandbox,
            sprite_sheet,
            glyph_brush,
            font_cache,
            screen_dimensions,
            mut iced_primitives,
            mut asset_textures,
            mut factory,
            queue,
            mut glyph_atlas,
            image_cache,
        ): Self::SystemData,
    ) {
        if sandbox.is_none() {
            log::warn!("No sandbox was found in resources, Iced UI will not be drawn.");
            return;
        }
        let mut sandbox = sandbox.unwrap();
        {
            let mut renderer =
                IcedRenderer::new(sprite_sheet, glyph_brush, font_cache, image_cache);

            if queue.is_none() {
                return;
            }
            let queue = queue.unwrap();
            let queue = *queue;
            {
                let glyph_atlas = glyph_atlas.0.get_or_insert_with(|| {
                    let (w, h) = renderer.glyph_brush.get_mut().texture_dimensions();
                    asset_textures.insert(create_glyph_texture(&mut *factory, queue, w, h))
                });
            }
            let reader = self
                .winit_reader_id
                .as_mut()
                .expect("Failed to get ReaderID: IcedUpdateSystem has not been setup.");
            let bounds: Size = [screen_dimensions.width(), screen_dimensions.height()].into();
            let cache = self.cache.take().unwrap();
            let mut user_interface =
                UserInterface::build(sandbox.view(), bounds, cache, &mut renderer);

            winit_events
                .read(reader)
                .filter_map(|winit_event| match winit_event {
                    // TODO: Propper handling of window events, using iced_winit::conversion
                    // Possible when Amethyst upgrades to winit 0.22
                    WinitEvent::WindowEvent {
                        event: WinitWindowEvent::Resized(size),
                        ..
                    } => Some(iced_native::Event::Window(
                        iced_native::window::Event::Resized {
                            width: size.width as u32,
                            height: size.height as u32,
                        },
                    )),
                    WinitEvent::WindowEvent {
                        event:
                            WinitWindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => Some(iced_native::Event::Mouse(
                        iced_native::mouse::Event::ButtonPressed(iced_native::mouse::Button::Left),
                    )),
                    WinitEvent::WindowEvent {
                        event:
                            WinitWindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    } => Some(iced_native::Event::Mouse(
                        iced_native::mouse::Event::ButtonReleased(iced_native::mouse::Button::Left),
                    )),
                    WinitEvent::WindowEvent {
                        event: WinitWindowEvent::CursorMoved { position, .. },
                        ..
                    } => Some(iced_native::Event::Mouse(
                        iced_native::mouse::Event::CursorMoved {
                            x: position.x as f32 * screen_dimensions.hidpi_factor() as f32,
                            y: position.y as f32 * screen_dimensions.hidpi_factor() as f32,
                        },
                    )),
                    _ => None,
                })
                .flat_map(|iced_event| user_interface.update(vec![iced_event], None, &renderer))
                .for_each(|ui_msg| ui_messages.single_write(ui_msg));

            *iced_primitives = user_interface.draw(&mut renderer);
            self.cache = Some(user_interface.into_cache());
        }
        let mut hasher = iced_native::Hasher::default();
        sandbox.hash_layout(&mut hasher);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let mut winit_event_channel = Write::<'_, EventChannel<WinitEvent>>::fetch(world);
        self.winit_reader_id = Some(winit_event_channel.register_reader());
    }
}

fn create_glyph_texture<B: Backend>(
    factory: &mut Factory<B>,
    queue: QueueId,
    width: u32,
    height: u32,
) -> Texture {
    use hal::format::{Component as C, Swizzle};
    TextureBuilder::new()
        .with_kind(hal::image::Kind::D2(width, height, 1, 1))
        .with_view_kind(hal::image::ViewKind::D2)
        .with_data_width(width)
        .with_data_height(height)
        .with_data(vec![R8Unorm { repr: [0] }; (width * height) as _])
        // This swizzle is required when working with `R8Unorm` on metal.
        // Glyph texture is biased towards 1.0 using "color_bias" attribute instead.
        .with_swizzle(Swizzle(C::Zero, C::Zero, C::Zero, C::R))
        .build(
            ImageState {
                queue,
                stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                access: hal::image::Access::SHADER_READ,
                layout: hal::image::Layout::General,
            },
            factory,
        )
        .map(B::wrap_texture)
        .expect("Failed to create glyph texture")
}
