use amethyst::{
    assets::Handle,
    assets::{AssetStorage, Loader},
    ecs::World,
    prelude::*,
    renderer::{rendy::hal::image::*, types::Backend, ImageFormat, Texture},
};

use iced_graphics::image;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct ImageCache {
    pub(crate) map: HashMap<String, Handle<Texture>>,
    pub(crate) dims: HashMap<String, (u32, u32)>,
}

impl ImageCache {
    pub fn dimensions(&self, handle: &image::Handle) -> (u32, u32) {
        match handle.data() {
            iced_native::image::Data::Path(pathbuf) => self
                .dims
                .get(&pathbuf.to_string_lossy().to_string())
                .unwrap_or(&(0, 0)).clone(),
            _ => (0, 0),
        }
    }

    pub fn get<B: Backend>(
        &mut self,
        handle: &image::Handle,
        world: &World,
    ) -> Option<Handle<Texture>> {
        match handle.data() {
            iced_native::image::Data::Path(pathbuf) => {
                let image = self
                    .map
                    .entry(pathbuf.to_string_lossy().to_string())
                    .or_insert_with(|| {
                        let loader = world.read_resource::<Loader>();
                        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                        loader.load(
                            pathbuf.to_string_lossy(),
                            ImageFormat::default(),
                            (),
                            &texture_storage,
                        )
                    })
                    .clone();

                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                if let Some(tex) = texture_storage.get(&image) {
                    match B::unwrap_texture(tex).unwrap().image().info().kind {
                        Kind::D2(width, height, _, _) => {
                            self.dims
                                .entry(pathbuf.to_string_lossy().to_string())
                                .or_insert_with(|| (width, height));
                            self.dims
                                .entry(pathbuf.to_string_lossy().to_string())
                                .and_modify(|x| {
                                    x.0 = width;
                                    x.1 = height
                                });
                        }
                        _ => {}
                    }
                }
                Some(image)
            }
            _ => None,
        }
    }
}
