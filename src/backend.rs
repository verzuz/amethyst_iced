use amethyst::assets::AssetStorage;
use amethyst::ecs::{Read, WriteExpect};
use amethyst::renderer::SpriteSheet;
use iced_graphics::Primitive;

use crate::resources::FontCache;
use crate::IcedGlyphBrush;
use iced_native::renderer::Renderer;

use std::cell::RefCell;

pub struct IcedRenderer<'a> {
    pub textures: Read<'a, AssetStorage<SpriteSheet>>,
    pub glyph_brush: RefCell<WriteExpect<'a, IcedGlyphBrush>>,
    pub font_cache: Read<'a, FontCache>,
}

impl<'a> IcedRenderer<'a> {
    pub fn new(
        textures: Read<'a, AssetStorage<SpriteSheet>>,
        glyph_brush: WriteExpect<'a, IcedGlyphBrush>,
        font_cache: Read<'a, FontCache>,
    ) -> Self {
        IcedRenderer {
            textures,
            glyph_brush: RefCell::new(glyph_brush),
            font_cache,
        }
    }
}

impl<'a> Renderer for IcedRenderer<'a> {
    type Output = Primitive;
    type Defaults = ();
}
