use iced::window::settings::PlatformSpecific;

use crate::app::App;

mod models;
mod app;
mod pages;
mod widgets;
mod resources;
mod utils;

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: (760.0, 570.0).into(),
        platform_specific: PlatformSpecific {
            application_id: "hadron".into(), ..Default::default()
        },
        ..Default::default()
    };

    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .window(window_settings)
        .run()
}
