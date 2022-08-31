use std::collections::VecDeque;

use chrono::prelude::*;
use iced::pure::widget::{Button, Column, Container, Row, Slider, Text};
use iced::pure::{Element, Sandbox};
use iced::{alignment, Alignment, Length, Settings};
use num_traits::cast::FromPrimitive;

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (600u32, 300u32);
    ShakuntalaDeviTrainer::run(settings).unwrap();
}

//Weekday does not implement Default so we can(t derive Default)
#[derive(Debug)]
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

impl Sandbox for ShakuntalaDeviTrainer {
    type Message = Message;

    fn new() -> Self {
        let (random_date, shakuntala_devi_answer, tips) =
            shakuntala_devi_trainer::random_date_with_tips(
                shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
                shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
            );
        Self {
            first_year: shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
            last_year: shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
            random_date: random_date.to_string(),
            week_day: shakuntala_devi_answer,
            already_pressed: Vec::new(),
            tips,
            hint: "Guess the day!".to_string(),
            start: instant::Instant::now(),
        }
    }

    fn title(&self) -> String {
        String::from("Shakuntala Devi trainer")
    }

    fn update(&mut self, message: Self::Message) {
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
    }

    fn view(&self) -> Element<Self::Message> {
        let reset_button = Column::new()
            .push(
                Button::new(
                    Text::new("Start new game")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::Reset)
                .style(style::Button::Start),
            )
            .padding(16);

        let column = |label, weekday, already_pressed| {
            Column::new()
                .push(if already_pressed {
                    Button::new(
                        Text::new(label)
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .size(16),
                    )
                    .padding(8)
                } else {
                    Button::new(
                        Text::new(label)
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .size(16),
                    )
                    .padding(8)
                    .on_press(Message::GuessDay(weekday))
                    .style(style::Button::Days)
                })
                .padding(1)
        };

        let result = Column::new()
            .push(Text::new(&self.hint).size(24))
            .padding(8);

        let random_date = Column::new()
            .push(Text::new(&self.random_date).size(48))
            .padding(8);

        let first_year = Column::new()
            .push(Text::new(&self.first_year.to_string()).size(12))
            .padding(0);

        let last_year = Column::new()
            .push(Text::new(&self.last_year.to_string()).size(12))
            .padding(0);

        let first_year_slider = Column::new()
            .push(Slider::new(
                shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
                self.first_year,
                Message::FirstYear,
            ))
            .padding(0);

        let last_year_slider = Column::new()
            .push(Slider::new(
                shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
                self.last_year,
                Message::LastYear,
            ))
            .padding(0);

        let weekday = Row::new()
            .push(column(
                "Monday",
                Weekday::Mon,
                self.already_pressed.contains(&Weekday::Mon),
            ))
            .push(column(
                "Tuesday",
                Weekday::Tue,
                self.already_pressed.contains(&Weekday::Tue),
            ))
            .push(column(
                "Wednesday",
                Weekday::Wed,
                self.already_pressed.contains(&Weekday::Wed),
            ))
            .push(column(
                "Thursday",
                Weekday::Thu,
                self.already_pressed.contains(&Weekday::Thu),
            ))
            .push(column(
                "Friday",
                Weekday::Fri,
                self.already_pressed.contains(&Weekday::Fri),
            ))
            .push(column(
                "Saturday",
                Weekday::Sat,
                self.already_pressed.contains(&Weekday::Sat),
            ))
            .push(column(
                "Sunday",
                Weekday::Sun,
                self.already_pressed.contains(&Weekday::Sun),
            ));

        let content = Column::new()
            .push(first_year_slider)
            .push(first_year)
            .push(last_year_slider)
            .push(last_year)
            .push(reset_button)
            .push(random_date)
            .push(weekday)
            .push(result)
            .align_items(Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Days,
        Start,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Days => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Start => Color::from_rgb(0.11, 0.67, 0.11),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
