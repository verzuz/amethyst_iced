use iced_graphics::{Background, Primitive};
use iced_native::slider::Renderer;
use iced_native::{Point, Rectangle};

use crate::backend::IcedRenderer;
use crate::primitive::AmethystIcedPrimitive;

use std::ops::RangeInclusive;

// Todo: replace those with styled values
const HANDLE_WIDTH: f32 = 20.;
const HANDLE_HEIGHT: f32 = 20.;

impl<'a> Renderer for IcedRenderer<'a> {
    type Style = ();

    fn height(&self) -> u32 {
        30
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        _cursor_position: Point,
        range: RangeInclusive<f32>,
        value: f32,
        _is_dragging: bool,
        _style: &Self::Style,
    ) -> Self::Output {
        // TODO: Handle style (knob should be custom, nine-patch & colors for rail)
        // TODO: Handle cursor position, is_dragging, etc ...

        let rail_y = bounds.y + (bounds.height / 2.0).round();

        let (rail_top, rail_bottom) = (
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y,
                    width: bounds.width,
                    height: 2.0,
                },
                background: Background::Color([1., 0., 0., 1.].into()),
                border_radius: 0,
                border_width: 1,
                border_color: [0.6, 0.6, 0.6, 0.5].into(),
            },
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y + 2.0,
                    width: bounds.width,
                    height: 2.0,
                },
                background: Background::Color([1., 0., 0., 1.].into()),
                border_radius: 0,
                border_width: 1,
                border_color: [0.6, 0.6, 0.6, 0.5].into(),
            },
        );

        let (range_start, range_end) = range.into_inner();

        let handle_offset = (bounds.width as f32 - HANDLE_WIDTH)
            * ((value - range_start) / (range_end - range_start).max(1.0));

        let knob = Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset.round(),
                y: rail_y - HANDLE_HEIGHT / 2.0,
                width: HANDLE_WIDTH,
                height: HANDLE_HEIGHT,
            },
            background: Background::Color([0., 1., 0., 1.].into()),
            border_radius: 0,
            border_width: 1,
            border_color: [0.6, 0.6, 0.6, 0.5].into(),
        };
        Primitive::Group {
            primitives: vec![rail_top, rail_bottom, knob],
        }
    }
}
