use iced_graphics::{Background, Primitive};
use iced_native::radio::Renderer;
use iced_native::Rectangle;

use crate::backend::IcedRenderer;

const SIZE: f32 = 28.0;

impl<'a> Renderer for IcedRenderer<'a> {
    type Style = ();
    const DEFAULT_SIZE: u16 = SIZE as u16;
    const DEFAULT_SPACING: u16 = 15;

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        _is_mouse_over: bool,
        label: Self::Output,
        _style: &Self::Style,
    ) -> Self::Output {
        // TODO: Style background color & radio color, outline
        println!("drawing radio");
        let background = Primitive::Quad {
            bounds: bounds,
            background: Background::Color([1., 1., 1., 1.].into()),
            border_radius: 0,
            border_width: 1,
            border_color: [0.6, 0.6, 0.6, 0.5].into(),
        };

        let selected = if is_selected {
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x + SIZE / 4.,
                    y: bounds.y + SIZE / 4.,
                    width: bounds.width - SIZE / 2.,
                    height: bounds.height - SIZE / 2.,
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
