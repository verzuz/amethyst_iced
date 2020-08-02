use iced_graphics::{image, Primitive};
use iced_native::image::Renderer;
use iced_native::mouse;
use iced_native::Layout;

use crate::backend::IcedRenderer;

impl<'a> Renderer for IcedRenderer<'a> {

    fn dimensions(&self, handle: &image::Handle) -> (u32, u32) {
        let x = self.image_cache.dimensions(handle);
        println!("{:?}",x);
        x
    }

    fn draw(&mut self, handle: image::Handle, layout: Layout<'_>) -> Self::Output {
        Primitive::Image {
            handle,
            bounds: layout.bounds(),
        }
    }
}
