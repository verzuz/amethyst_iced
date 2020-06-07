use iced_native::progress_bar::Renderer;
use iced_native::Rectangle;

use crate::backend::IcedRenderer;
use iced_graphics::{Background, Primitive};

impl<'a> Renderer for IcedRenderer<'a> {
    type Style = ();

    const DEFAULT_HEIGHT: u16 = 30;

    fn draw(
        &self,
        bounds: Rectangle,
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        _style_sheet: &Self::Style,
    ) -> Self::Output {
        let (range_start, range_end) = range.into_inner();
        let active_progress_width =
            bounds.width * ((value - range_start) / (range_end - range_start).max(1.0));

        let background = Primitive::Quad {
            bounds: bounds,
            background: Background::Color([1.0, 1.0, 1.0, 1.0].into()),
            border_radius: 0,
            border_width: 1,
            border_color: [0.6, 0.6, 0.6, 0.5].into(),
        };

        if active_progress_width > 0.0 {
            let bar = Primitive::Quad {
                bounds: Rectangle {
                    width: active_progress_width,
                    ..bounds
                },
                background: Background::Color([1.0, 1.0, 0.0, 0.0].into()),
                border_radius: 0,
                border_width: 1,
                border_color: [0.6, 0.6, 0.6, 0.5].into(),
            };

            Primitive::Group {
                primitives: vec![background, bar],
            }
        } else {
            background
        }
    }
}
