
use iced::theme::Slider;
use iced::widget::{button, container, slider, text};
use iced::{application, color, Background, Color, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0xff, 0xff, 0xff),
            text_color: color!(0x0, 0x0, 0x0),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}

impl slider::StyleSheet for Theme {
    type Style = Slider;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (color!(0xda, 0xda, 0xda), color!(0xda, 0xda, 0xda)),
            handle: {
                slider::Handle {
                    shape: iced::widget::slider::HandleShape::Rectangle {
                        width: 8,
                        border_radius: 12.0,
                    },
                    color: color!(0x9c, 0x9c, 0x9c),
                    border_width: (1.0),
                    border_color: color!(0x0, 0x0, 0x0),
                }
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (color!(0xda, 0xda, 0xda), color!(0xda, 0xda, 0xda)),
            handle: {
                slider::Handle {
                    shape: iced::widget::slider::HandleShape::Rectangle {
                        width: 8,
                        border_radius: 12.0,
                    },
                    color: color!(0xfc, 0xfc, 0xfc),
                    border_width: (1.0),
                    border_color: color!(0x0, 0x0, 0x0),
                }
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (color!(0xda, 0xda, 0xda), color!(0xda, 0xda, 0xda)),
            handle: {
                slider::Handle {
                    shape: iced::widget::slider::HandleShape::Rectangle {
                        width: 8,
                        border_radius: 12.0,
                    },
                    color: color!(0xda, 0xda, 0xda),
                    border_width: (1.0),
                    border_color: color!(0x0, 0x0, 0x0),
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    #[allow(dead_code)]
    Bordered,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Bordered => container::Appearance::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Days,
    Start,
}

impl button::StyleSheet for Theme {
    type Style = Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Days => button::Appearance {
                background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..Default::default()
            },
            Button::Start => button::Appearance {
                background: Color::from_rgb(0.11, 0.67, 0.11).into(),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.7,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.7,
                ..active.text_color
            },
            ..active
        }
    }
}