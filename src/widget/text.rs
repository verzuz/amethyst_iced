use crate::{backend::IcedRenderer, primitive::AmethystIcedPrimitive};
use glyph_brush::{rusttype::Scale, FontId, GlyphCruncher, Section};
use iced_native::widget::text::Renderer as TextRenderer;
use iced_native::{Color, Font, HorizontalAlignment, Rectangle, Size, VerticalAlignment};
use iced_graphics::Primitive;

impl<'a> TextRenderer for IcedRenderer<'a> {
    type Font = Font;

    const DEFAULT_SIZE: u16 = 16;

    fn measure(&self, content: &str, size: u16, font: Font, bounds: Size) -> (f32, f32) {
        let font_id = match font {
            Font::Default => FontId::default(),
            Font::External { name, .. } => {
                self.font_cache.get_id(name).cloned().unwrap_or_default()
            }
        };

        if let Some(measurement) = self.glyph_brush.borrow_mut().glyph_bounds(Section {
            font_id,
            text: content,
            scale: Scale::uniform(size as f32),
            bounds: (bounds.width, bounds.height),
            ..Default::default()
        }) {
            (measurement.width(), measurement.height())
        } else {
            (100., 100.)
        }
    }

    fn draw(
        &mut self,
        _defaults: &Self::Defaults,
        bounds: Rectangle,
        content: &str,
        size: u16,
        font: Font,
        color: Option<Color>,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self::Output {
        let color = color.unwrap_or(Color::WHITE);
        let color = [color.r, color.g, color.b, color.a];

        let font_id = match font {
            Font::Default => FontId::default(),
            Font::External { name, .. } => {
                self.font_cache.get_id(name).cloned().unwrap_or_default()
            }
        };

        Primitive::Text {
            bounds: bounds,
            content: content.to_string(),
            size:size as f32,
            color:color.into(),
            horizontal_alignment:horizontal_alignment,
            vertical_alignment: vertical_alignment,
            font: Font::Default,
        }
    }
}
