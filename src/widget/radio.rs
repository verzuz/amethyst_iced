use iced_native::radio::Renderer;
use iced_native::Rectangle;

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;

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
        let background = AmethystIcedPrimitive::Quad(bounds, Some([1., 1., 1., 1.].into()));
        let selected = if is_selected {
            AmethystIcedPrimitive::Quad(
                Rectangle {
                    x: bounds.x + SIZE / 4.,
                    y: bounds.y + SIZE / 4.,
                    width: bounds.width - SIZE / 2.,
                    height: bounds.height - SIZE / 2.,
                },
                Some([0., 1., 0., 1.].into()),
            )
        } else {
            AmethystIcedPrimitive::None
        };
        AmethystIcedPrimitive::Group(vec![background, selected, label])
    }
}
