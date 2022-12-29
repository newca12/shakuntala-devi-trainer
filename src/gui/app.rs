use crate::gui::theme::Theme;
use crate::gui::widget::Element;

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
    settings.window.icon =
        match image::load_from_memory(include_bytes!("../../assets/calendar.png")) {
            Ok(buffer) => {
                let buffer = buffer.to_rgba8();
                let width = buffer.width();
                let height = buffer.height();
                let dynamic_image = image::DynamicImage::ImageRgba8(buffer);
                match iced::window::icon::Icon::from_rgba(dynamic_image.into_bytes(), width, height)
                {
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
        .style(crate::gui::theme::Button::Start),]
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
                .style(crate::gui::theme::Button::Days)
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
