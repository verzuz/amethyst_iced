use amethyst::{
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        //types::DefaultBackend,
        rendy::util::vulkan::Backend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Error,
};
use amethyst_iced::{
    Align, Color, Column, Container, Element, IcedBundle, IcedUI, Image, Length, Sandbox,
    SandboxContainer, Text,
};
use iced_graphics::image;

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<Backend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.1, 0.1, 0.1, 1.0]),
                )
                .with_plugin(IcedUI::default()),
        )?
        .with_bundle(IcedBundle::<ImageUIState, Backend>::default())?;

    let mut game = Application::new(assets, ImageState::default(), game_data)?;
    game.run();

    Ok(())
}

struct ImageUIState {
    image: image::Handle,
}

#[derive(Default, Debug)]
struct ImageState;

impl SimpleState for ImageState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let image = image::Handle::from_path("texture/test.png");
        world.insert(SandboxContainer::new(ImageUIState { image }))
    }
}

impl Sandbox for ImageUIState {
    type UIMessage = u32;
    type GameMessage = ();

    fn view(&mut self) -> Element<Self::UIMessage> {
        let col = Column::new()
            .spacing(5)
            .align_items(Align::Center)
            .push(Text::new("Hello world in red").color(Color::from_rgb(1., 0., 0.)))
            .push(Image::new(self.image.clone()))
            .push(Image::new(self.image.clone()));

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
