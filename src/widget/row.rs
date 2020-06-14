use iced_graphics::Primitive;
use iced_native::row::Renderer;
use iced_native::{Element, Layout, Point};

use crate::backend::IcedRenderer;

impl<'a> Renderer for IcedRenderer<'a> {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        children: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output {
        Primitive::Group {
            primitives: children
                .iter()
                .zip(layout.children())
                .map(|(child, layout)| child.draw(self, defaults, layout, cursor_position))
                .collect(),
        }
    }
}
