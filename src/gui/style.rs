use iced::{
    border::Radius,
    widget::{
        button, container,
        slider::{self},
    },
    Border, Color, Theme,
};

pub fn main_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb8(0xFF, 0xFE, 0xF0).into()),
        ..Default::default()
    }
}

pub fn rounded(theme: &Theme, status: button::Status) -> button::Style {
    let radius = Radius {
        top_left: 12.0,
        top_right: 12.0,
        bottom_left: 12.0,
        bottom_right: 12.0,
    };
    button::Style {
        text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
        border: Border {
            radius,
            width: 1.0,
            color: Color::from_rgb(0.53, 0.0, 0.85),
        },
        ..button::primary(theme, status)
    }
}

pub fn button_start(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        iced::widget::button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.11, 0.67, 0.11).into()),
            text_color: Color::WHITE,
            ..rounded(theme, status)
        },
        _ => button::Style {
            background: Some(Color::from_rgb(0.11, 0.67, 0.11).into()),
            ..rounded(theme, status)
        },
    }
}

pub fn button_day(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        iced::widget::button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.11, 0.42, 0.87).into()),
            text_color: Color::WHITE,
            ..rounded(theme, status)
        },
        iced::widget::button::Status::Disabled => button::Style {
            background: Some(Color::from_rgb(00.59, 0.67, 0.91).into()),
            ..rounded(theme, status)
        },
        _ => button::Style {
            background: Some(Color::from_rgb(0.11, 0.42, 0.87).into()),
            ..rounded(theme, status)
        },
    }
}

pub fn button_menu(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        iced::widget::button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.53, 0.0, 0.85).into()),
            text_color: Color::WHITE,
            ..rounded(theme, status)
        },
        _ => button::Style {
            background: Some(Color::from_rgb(0.53, 0.0, 0.85).into()),
            ..rounded(theme, status)
        },
    }
}

pub fn button_menu_inactive(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: Color::BLACK,
        ..rounded(theme, status)
    }
}

pub fn slider_style(_theme: &Theme, status: slider::Status) -> slider::Style {
    match status {
        iced::widget::slider::Status::Active => slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                    Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                ),
                width: 2.0,
                border: Border {
                    radius: 2.0.into(),
                    width: 0.0,
                    color: Color::from_rgba8(0x0, 0x0, 0x0, 1.0),
                },
            },
            handle: slider::Handle {
                shape: iced::widget::slider::HandleShape::Rectangle {
                    width: 8,
                    border_radius: 12.0.into(),
                },
                background: Color::from_rgba8(0x9c, 0x9c, 0x9c, 1.0).into(),
                border_color: Color::from_rgba8(0x0, 0x0, 0x0, 1.0),
                border_width: 0.0,
            },
        },
        iced::widget::slider::Status::Hovered => slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                    Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                ),
                width: 2.0,
                border: Border {
                    radius: 12.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
            handle: slider::Handle {
                shape: iced::widget::slider::HandleShape::Rectangle {
                    width: 8,
                    border_radius: 12.0.into(),
                },
                background: Color::from_rgba8(0xfc, 0xfc, 0xfc, 1.0).into(),
                border_color: Color::from_rgba8(0x0, 0x0, 0x0, 1.0),
                border_width: 1.0,
            },
        },
        iced::widget::slider::Status::Dragged => slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                    Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                ),
                width: 2.0,
                border: Border {
                    radius: 2.0.into(),
                    width: 0.0,
                    color: Color::from_rgba8(0x0, 0x0, 0x0, 1.0),
                },
            },
            handle: slider::Handle {
                shape: iced::widget::slider::HandleShape::Rectangle {
                    width: 8,
                    border_radius: 12.0.into(),
                },
                background: Color::from_rgba8(0xda, 0xda, 0xda, 1.0).into(),
                border_color: Color::from_rgba8(0x0, 0x0, 0x0, 1.0),
                border_width: 1.0,
            },
        },
    }
}
