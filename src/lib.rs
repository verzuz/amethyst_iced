mod backend;
mod bundle;
mod custom_widget;
mod pass;
mod pipelines;
mod plugin;
mod resources;
pub mod sandbox;
pub mod style;
mod systems;
mod uniform;
mod vertex;

pub mod widget;

pub use bundle::IcedBundle;
pub use custom_widget::*;
pub use plugin::IcedUI;
pub use sandbox::{Element, Sandbox, SandboxContainer};

// Conveniently re-exports iced's Widget types
pub use iced_native::{
    button::State as ButtonState, pane_grid, slider::State as SliderState, Align, Color, Font,
    HorizontalAlignment, Length, Text, VerticalAlignment,
};

pub use resources::*;
pub use style::*;
pub use widget::*;

pub type IcedGlyphBrush = glyph_brush::GlyphBrush<'static, (u32, Vec<crate::vertex::TextVertex>)>;
