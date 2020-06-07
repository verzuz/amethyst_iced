use iced_graphics::{Background, Primitive};
use iced_native::checkbox::Renderer;
use iced_native::Rectangle;

use crate::backend::IcedRenderer;
//use crate::primitive::AmethystIcedPrimitive;

impl<'a> Renderer for IcedRenderer<'a> {
    const DEFAULT_SIZE: u16 = 20;
    const DEFAULT_SPACING: u16 = 20;

    type Style = ();

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_checked: bool,
        _is_mouse_over: bool,
        label: Self::Output,
        _style: &Self::Style,
    ) -> Self::Output {
        // TODO: Style background color & radio color, outline
        println!("drawing radio");
        let background = AmethystIcedPrimitive::Quad(bounds, Some([1., 1., 1., 1.].into()));
        let selected = if is_checked {
            let default_size = Self::DEFAULT_SIZE as f32;
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x + default_size / 4.,
                    y: bounds.y + default_size / 4.,
                    width: bounds.width - default_size / 2.,
                    height: bounds.height - default_size / 2.,
                },
                background: Background::Color([0., 1., 0., 1.].into()),
                border_radius: 0,
                border_width: 1,
                border_color: [0.6, 0.6, 0.6, 0.5].into(),
            }
        } else {
            Primitive::None
        };
        Primitive::Group {
            primitives: vec![background, selected, label],
        }
    }
}
