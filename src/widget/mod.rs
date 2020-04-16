mod button;
mod checkbox;
mod column;
mod container;
mod radio;
mod row;
mod slider;
mod space;
mod text;

use crate::backend::IcedRenderer;

pub type Button<'a, 'r, Message> = iced_native::Button<'a, Message, IcedRenderer<'r>>;
pub type Checkbox<'a, Message> = iced_native::Checkbox<Message, IcedRenderer<'a>>;
pub type Container<'a, 'r, Message> = iced_native::Container<'a, Message, IcedRenderer<'r>>;
pub type Column<'a, 'r, Message> = iced_native::Column<'a, Message, IcedRenderer<'r>>;
pub type Slider<'a, 'r, Message> = iced_native::Slider<'a, Message, IcedRenderer<'r>>;
pub type Space = iced_native::Space;
pub type Radio<'a, Message> = iced_native::Radio<Message, IcedRenderer<'a>>;
pub type Row<'a, 'r, Message> = iced_native::Row<'a, Message, IcedRenderer<'r>>;
