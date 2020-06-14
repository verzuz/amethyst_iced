use amethyst::{
    assets::Processor,
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
    renderer::types::Backend,
    shrev::EventChannel,
    ui::FontAsset,
    Error,
};
use glyph_brush::GlyphBrushBuilder;

use crate::{
    sandbox::Sandbox,
    systems::{IcedDrawSystem, IcedInteropSystem, LoadFontToCacheSystem},
    IcedGlyphBrush,
};

use iced_graphics::Primitive;

pub struct IcedBundle<S: Sandbox, B: Backend> {
    _sandbox: std::marker::PhantomData<S>,
    _backend: std::marker::PhantomData<B>,
}

impl<S: Sandbox, B: Backend> Default for IcedBundle<S, B> {
    fn default() -> Self {
        IcedBundle::new()
    }
}

impl<S: Sandbox, B: Backend> IcedBundle<S, B> {
    /// Creates a new IcedBundle containing a Sandboxed application
    pub fn new() -> Self {
        IcedBundle {
            _sandbox: std::marker::PhantomData,
            _backend: std::marker::PhantomData,
        }
    }
}

impl<'a, 'b, S: Sandbox, B: Backend> SystemBundle<'a, 'b> for IcedBundle<S, B> {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Creates communication channels for the Sandbox
        world.insert(EventChannel::<S::UIMessage>::default());
        world.insert(EventChannel::<S::GameMessage>::default());
        world.insert(Primitive::default());
        let square_ttf: &[u8] = include_bytes!("../font/square.ttf");
        world.insert::<IcedGlyphBrush>(GlyphBrushBuilder::using_font_bytes(square_ttf).build());

        // Adds Iced-related systems
        dispatcher.add(IcedInteropSystem::<S>::default(), "iced_interop", &[]);
        dispatcher.add(
            IcedDrawSystem::<S, B>::default(),
            "iced_draw",
            &["iced_interop"],
        );
        dispatcher.add(Processor::<FontAsset>::new(), "iced_font_processor", &[]);
        dispatcher.add(
            LoadFontToCacheSystem::default(),
            "iced_load_font_to_cache",
            &[],
        );
        Ok(())
    }
}
