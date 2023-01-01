use iced::{Application, Settings};

pub mod app;
pub mod common;
pub mod theme;
pub mod widget;

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (600u32, 350u32);
    settings.window.icon = match image::load_from_memory(include_bytes!("../assets/calendar.png")) {
        Ok(buffer) => {
            let buffer = buffer.to_rgba8();
            let width = buffer.width();
            let height = buffer.height();
            let dynamic_image = image::DynamicImage::ImageRgba8(buffer);
            match iced::window::icon::Icon::from_rgba(dynamic_image.into_bytes(), width, height) {
                Ok(icon) => Some(icon),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };
    app::ShakuntalaDeviTrainer::run(settings).unwrap();
}
