use amethyst::assets::AssetStorage;
use amethyst::ecs::{Read, Write, WriteExpect};
use amethyst::renderer::{SpriteSheet, Texture};
use iced_graphics::Primitive;

use crate::resources::{FontCache, ImageCache};
use crate::IcedGlyphBrush;
use iced_native::renderer::Renderer;

use std::cell::RefCell;

pub struct IcedRenderer<'a> {
    pub textures: Read<'a, AssetStorage<SpriteSheet>>,
    pub glyph_brush: RefCell<WriteExpect<'a, IcedGlyphBrush>>,
    pub font_cache: Read<'a, FontCache>,
    pub image_cache: Write<'a, ImageCache>,
}

impl<'a> IcedRenderer<'a> {
    pub fn new(
        textures: Read<'a, AssetStorage<SpriteSheet>>,
        glyph_brush: WriteExpect<'a, IcedGlyphBrush>,
        font_cache: Read<'a, FontCache>,
        image_cache: Write<'a, ImageCache>,
    ) -> Self {
        IcedRenderer {
            textures,
            glyph_brush: RefCell::new(glyph_brush),
            font_cache,
            image_cache,
        }
    }
}

impl<'a> Renderer for IcedRenderer<'a> {
    type Output = Primitive;
    type Defaults = ();
}
