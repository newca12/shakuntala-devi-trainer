use std::collections::VecDeque;

use chrono::prelude::*;
use iced::{
    button, slider, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row,
    Sandbox, Settings, Slider, Text,
};
use num_traits::cast::FromPrimitive;

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (600u32, 300u32);
    ShakuntalaDeviTrainer::run(settings).unwrap();
}

//Weekday does not implement Default so we can(t derive Default)
#[derive(Debug)]
struct ShakuntalaDeviTrainer {
    first_year_slider: slider::State,
    first_year: u32,
    last_year_slider: slider::State,
    last_year: u32,
    reset: button::State,
    monday: button::State,
    tuesday: button::State,
    wednesday: button::State,
    thursday: button::State,
    friday: button::State,
    saturday: button::State,
    sunday: button::State,
    random_date: String,
    week_day: Weekday,
    already_pressed: Vec<Weekday>,
    tips: VecDeque<String>,
    hint: String,
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
            first_year_slider: slider::State::new(),
            first_year: shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
            last_year_slider: slider::State::new(),
            last_year: shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
            reset: button::State::new(),
            monday: button::State::new(),
            tuesday: button::State::new(),
            wednesday: button::State::new(),
            thursday: button::State::new(),
            friday: button::State::new(),
            saturday: button::State::new(),
            sunday: button::State::new(),
            random_date: random_date.to_string(),
            week_day: shakuntala_devi_answer,
            already_pressed: Vec::new(),
            tips,
            hint: "Guess the day!".to_string(),
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
                        "Congratulation ! You found {} after {} guess",
                        guess_day.to_string(),
                        tries
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

    fn view(&mut self) -> Element<Self::Message> {
        let reset_button = Column::new()
            .push(
                Button::new(
                    &mut self.reset,
                    Text::new("Start new game")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::Reset)
                .style(style::Button::Start),
            )
            .padding(16);

        let column = |state, label, weekday, already_pressed| {
            Column::new()
                .push(if already_pressed {
                    Button::new(
                        state,
                        Text::new(label)
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .size(16),
                    )
                    .padding(8)
                } else {
                    Button::new(
                        state,
                        Text::new(label)
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .size(16),
                    )
                    .padding(8)
                    .on_press(Message::GuessDay(weekday))
                    .style(style::Button::Days)
                })
                .padding(1)
        };

        let result = Column::new()
            .push(Text::new(&self.hint).size(32))
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
                &mut self.first_year_slider,
                shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
                self.first_year,
                Message::FirstYear,
            ))
            .padding(0);

        let last_year_slider = Column::new()
            .push(Slider::new(
                &mut self.last_year_slider,
                shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
                self.last_year,
                Message::LastYear,
            ))
            .padding(0);

        let weekday = Row::new()
            .push(column(
                &mut self.monday,
                "Monday",
                Weekday::Mon,
                self.already_pressed.contains(&Weekday::Mon),
            ))
            .push(column(
                &mut self.tuesday,
                "Tuesday",
                Weekday::Tue,
                self.already_pressed.contains(&Weekday::Tue),
            ))
            .push(column(
                &mut self.wednesday,
                "Wednesday",
                Weekday::Wed,
                self.already_pressed.contains(&Weekday::Wed),
            ))
            .push(column(
                &mut self.thursday,
                "Thursday",
                Weekday::Thu,
                self.already_pressed.contains(&Weekday::Thu),
            ))
            .push(column(
                &mut self.friday,
                "Friday",
                Weekday::Fri,
                self.already_pressed.contains(&Weekday::Fri),
            ))
            .push(column(
                &mut self.saturday,
                "Saturday",
                Weekday::Sat,
                self.already_pressed.contains(&Weekday::Sat),
            ))
            .push(column(
                &mut self.sunday,
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
            .align_items(Align::Center);

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
