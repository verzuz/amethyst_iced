use iced_graphics::Primitive;
use iced_native::{space::Renderer, Rectangle};

use crate::backend::IcedRenderer;

impl<'a> Renderer for IcedRenderer<'a> {
    fn draw(&mut self, _bounds: Rectangle) -> Self::Output {
        Primitive::None
    }
}
