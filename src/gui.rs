use self::theme::Theme;
use self::widget::Element;

use std::collections::VecDeque;

use chrono::prelude::*;
use num_traits::cast::FromPrimitive;

use iced::{
    alignment, executor,
    widget::{button, column, row, text, Container, Slider},
    Alignment, Application, Length, Settings,
};

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (600u32, 300u32);
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
    ShakuntalaDeviTrainer::run(settings).unwrap();
}

//Weekday does not implement Default so we can't derive Default
#[derive(Debug, Clone)]
struct ShakuntalaDeviTrainer {
    first_year: u32,
    last_year: u32,
    random_date: String,
    week_day: Weekday,
    already_pressed: Vec<Weekday>,
    tips: VecDeque<String>,
    hint: String,
    start: instant::Instant,
}

#[derive(Debug, Clone)]
enum Message {
    GuessDay(Weekday),
    Reset,
    FirstYear(u32),
    LastYear(u32),
}

impl Application for ShakuntalaDeviTrainer {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn theme(&self) -> Theme {
        Theme::default()
    }

    fn new(_flags: ()) -> (ShakuntalaDeviTrainer, iced::Command<Message>) {
        let (random_date, shakuntala_devi_answer, tips) =
            shakuntala_devi_trainer::random_date_with_tips(
                shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
                shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
            );
        (
            Self {
                first_year: shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
                last_year: shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
                random_date: random_date.to_string(),
                week_day: shakuntala_devi_answer,
                already_pressed: Vec::new(),
                tips,
                hint: "Guess the day!".to_string(),
                start: instant::Instant::now(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "{} {}",
            "Shakuntala Devi trainer",
            env!("CARGO_PKG_VERSION")
        )
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::GuessDay(guess_day) => {
                self.already_pressed.push(guess_day);
                let result = if guess_day == self.week_day {
                    let tries = self.already_pressed.len();
                    //enum are not iterable https://github.com/rust-lang/rfcs/issues/284
                    for n in 0..=6 {
                        self.already_pressed.push(Weekday::from_u32(n).unwrap());
                    }
                    format!(
                        "Congratulation ! You found {} after {} guess in {:#?}",
                        guess_day,
                        tries,
                        self.start.elapsed()
                    )
                } else {
                    match self.tips.pop_front() {
                        Some(tips) => format!("tips: {:#?}", tips),
                        None => "Sorry, no more tips".to_string(),
                    }
                };
                self.hint = result;
            }

            Message::Reset => {
                let (random_date, shakuntala_devi_answer, tips) =
                    shakuntala_devi_trainer::random_date_with_tips(self.first_year, self.last_year);
                self.week_day = shakuntala_devi_answer;
                self.random_date = random_date.to_string();
                self.tips = tips;
                self.hint = "Guess the day!".to_string();
                self.already_pressed = Vec::new();
                self.start = instant::Instant::now();
            }

            Message::FirstYear(first_year) => {
                if first_year < self.last_year {
                    self.first_year = first_year;
                }
            }

            Message::LastYear(last_year) => {
                if last_year > self.first_year {
                    self.last_year = last_year;
                }
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        let reset_button = column![button(
            text("Start new game")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(16),
        )
        .padding(8)
        .on_press(Message::Reset)
        .style(theme::Button::Start),]
        .padding(16);

        let column = |label, weekday, already_pressed| {
            column![if already_pressed {
                button(
                    text(label)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .size(16),
                )
                .padding(8)
            } else {
                button(
                    text(label)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(weekday))
                .style(theme::Button::Days)
            }]
            .padding(1)
        };

        let result = column![text(&self.hint).size(24)].padding(8);

        let random_date = column![text(&self.random_date).size(48)].padding(8);

        let first_year = column![text(self.first_year.to_string()).size(12)].padding(0);

        let last_year = column![text(self.last_year.to_string()).size(12)].padding(0);

        let first_year_slider = column![Slider::new(
            shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
            self.first_year,
            Message::FirstYear,
        )]
        .padding(0);

        let last_year_slider = column![Slider::new(
            shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
            self.last_year,
            Message::LastYear,
        )]
        .padding(0);

        let weekday = row![
            column(
                "Monday",
                Weekday::Mon,
                self.already_pressed.contains(&Weekday::Mon),
            ),
            column(
                "Tuesday",
                Weekday::Tue,
                self.already_pressed.contains(&Weekday::Tue),
            ),
            column(
                "Wednesday",
                Weekday::Wed,
                self.already_pressed.contains(&Weekday::Wed),
            ),
            column(
                "Thursday",
                Weekday::Thu,
                self.already_pressed.contains(&Weekday::Thu),
            ),
            column(
                "Friday",
                Weekday::Fri,
                self.already_pressed.contains(&Weekday::Fri),
            ),
            column(
                "Saturday",
                Weekday::Sat,
                self.already_pressed.contains(&Weekday::Sat),
            ),
            column(
                "Sunday",
                Weekday::Sun,
                self.already_pressed.contains(&Weekday::Sun),
            )
        ];

        let content = column![
            first_year_slider,
            first_year,
            last_year_slider,
            last_year,
            reset_button,
            random_date,
            weekday,
            result
        ]
        .align_items(Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// Always import widget types from this module since it
// uses our custom theme instead of the built-in iced::Theme.
// Otherwise you will get compilation errors since iced::Element
// expects use of iced::Theme by default.
mod widget {
    #![allow(dead_code)]
    use crate::gui::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}

mod theme {
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
}
