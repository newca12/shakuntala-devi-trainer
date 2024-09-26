pub mod app;
pub mod common;
pub mod style;

use self::app::ShakuntalaDeviTrainer;
use iced::Size;

pub fn run_gui() {
    let app = iced::application(
        ShakuntalaDeviTrainer::title,
        ShakuntalaDeviTrainer::update,
        ShakuntalaDeviTrainer::view,
    )
    .theme(ShakuntalaDeviTrainer::theme)
    .settings(iced::Settings {
        ..Default::default()
    })
    .window(iced::window::Settings {
        size: Size::new(600f32, 350f32),
        min_size: Some(Size::new(600f32, 350f32)),
        exit_on_close_request: true,
        icon: match image::load_from_memory(include_bytes!("../assets/calendar.png")) {
            Ok(buffer) => {
                let buffer = buffer.to_rgba8();
                let width = buffer.width();
                let height = buffer.height();
                let dynamic_image = image::DynamicImage::ImageRgba8(buffer);
                match iced::window::icon::from_rgba(dynamic_image.into_bytes(), width, height) {
                    Ok(icon) => Some(icon),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        },
        ..Default::default()
    });
    fn shakuntala_devi_trainer() -> (ShakuntalaDeviTrainer, iced::Task<app::Message>) {
        ShakuntalaDeviTrainer::new()
    }
    app.run_with(shakuntala_devi_trainer).unwrap();
}
