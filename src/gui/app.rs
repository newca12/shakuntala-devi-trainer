use crate::gui::theme::Theme;
use crate::gui::widget::Element;

use std::collections::VecDeque;

use chrono::prelude::*;
use num_traits::cast::FromPrimitive;

use iced::{
    alignment, executor,
    widget::{button, column, row, text, Container, Slider},
    Alignment, Application, Length,
};
use shakuntala_devi_trainer::T2;

use crate::gui::common::Screen;

//Weekday does not implement Default so we can't derive Default
#[derive(Debug, Clone)]
pub(crate) struct ShakuntalaDeviTrainer {
    screen: Screen,
    first_year: u32,
    last_year: u32,
    random_date: NaiveDate,
    week_day: Weekday,
    game_answers: Vec<Weekday>,
    t2_answers: Vec<i32>,
    tips: VecDeque<String>,
    hint: String,
    start: instant::Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    GuessDay(Weekday),
    GuessStep1(i32),
    Reset,
    FirstYear(u32),
    LastYear(u32),
    GameMode,
    TrainingStep1Mode,
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
                screen: Screen::Game,
                first_year: shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
                last_year: shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
                random_date,
                week_day: shakuntala_devi_answer,
                game_answers: Vec::new(),
                t2_answers: Vec::new(),
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
                self.game_answers.push(guess_day);
                let result = if guess_day == self.week_day {
                    let tries = self.game_answers.len();
                    //enum are not iterable https://github.com/rust-lang/rfcs/issues/284
                    for n in 0..=6 {
                        self.game_answers.push(Weekday::from_u32(n).unwrap());
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
                //self.screen = Screen::Game;
                let (random_date, shakuntala_devi_answer, tips) =
                    shakuntala_devi_trainer::random_date_with_tips(self.first_year, self.last_year);
                self.week_day = shakuntala_devi_answer;
                self.random_date = random_date;
                self.tips = tips;
                self.hint = if self.screen == Screen::Game {
                    "Guess the day!".to_string()
                } else {
                    "Which entry is the good one ?".to_string()
                };
                self.game_answers = Vec::new();
                self.start = instant::Instant::now();
            }

            Message::TrainingStep1Mode => {
                self.screen = Screen::TrainingStep1;
                self.hint = "Which entry is the good one ?".to_string()
            }

            Message::GameMode => {
                self.screen = Screen::Game;
                self.hint = "Guess the day!".to_string()
            }

            Message::GuessStep1(guess) => {
                self.t2_answers.push(guess);
                if T2[self.random_date.month0() as usize] == guess {
                    self.hint = "Congratulation !".to_string()
                } else {
                    self.hint = "Try again".to_string()
                };
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

        let menu_game = column![button(
            text("DAY TRAINING MODE")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(16),
        )
        .padding(8)
        .on_press(Message::GameMode)
        .style(if self.screen == Screen::Game {
            crate::gui::theme::Button::MenuActive
        } else {
            crate::gui::theme::Button::MenuInactive
        }),]
        .padding(16);
        let menu_month_table = column![button(
            text("MONTH TABLE TRAINING MODE")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(16),
        )
        .padding(8)
        .on_press(Message::TrainingStep1Mode)
        .style(if self.screen == Screen::TrainingStep1 {
            crate::gui::theme::Button::MenuActive
        } else {
            crate::gui::theme::Button::MenuInactive
        }),]
        .padding(16);

        let menu = row![menu_game, menu_month_table];

        let column_weekday = |label, weekday, already_pressed| {
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

        let column_t2 = |label, weekday, already_pressed| {
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
                .on_press(Message::GuessStep1(weekday))
                .style(crate::gui::theme::Button::Days)
            }]
            .padding(1)
        };

        let result = column![text(&self.hint).size(24)].padding(8);

        let random_date = {
            let month = Month::from_u32(self.random_date.month()).unwrap().name();
            let date = format!(
                "{} {} {}",
                self.random_date.day(),
                month,
                self.random_date.year()
            );
            match self.screen {
                Screen::Game => column![text(date).size(48)].padding(8),
                Screen::TrainingStep1 => column![text(month).size(48)].padding(8),
            }
        };

        let first_year = column![text(self.first_year.to_string()).size(12)];

        let last_year = column![text(self.last_year.to_string()).size(12)];

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
            column_weekday(
                "Monday",
                Weekday::Mon,
                self.game_answers.contains(&Weekday::Mon),
            ),
            column_weekday(
                "Tuesday",
                Weekday::Tue,
                self.game_answers.contains(&Weekday::Tue),
            ),
            column_weekday(
                "Wednesday",
                Weekday::Wed,
                self.game_answers.contains(&Weekday::Wed),
            ),
            column_weekday(
                "Thursday",
                Weekday::Thu,
                self.game_answers.contains(&Weekday::Thu),
            ),
            column_weekday(
                "Friday",
                Weekday::Fri,
                self.game_answers.contains(&Weekday::Fri),
            ),
            column_weekday(
                "Saturday",
                Weekday::Sat,
                self.game_answers.contains(&Weekday::Sat),
            ),
            column_weekday(
                "Sunday",
                Weekday::Sun,
                self.game_answers.contains(&Weekday::Sun),
            )
        ];

        let t3 = row![
            column_t2("0", 0, self.t2_answers.contains(&0),),
            column_t2("1", 1, self.t2_answers.contains(&1),),
            column_t2("2", 2, self.t2_answers.contains(&2),),
            column_t2("3", 3, self.t2_answers.contains(&3),),
            column_t2("4", 4, self.t2_answers.contains(&4),),
            column_t2("5", 5, self.t2_answers.contains(&5),),
            column_t2("6", 6, self.t2_answers.contains(&6),),
        ];

        let (main_screen, secondary_screen) = match self.screen {
            Screen::Game => (random_date, weekday),
            Screen::TrainingStep1 => (random_date, t3),
        };

        let container_slider = Container::new(
            first_year_slider
                .push(first_year)
                .align_items(Alignment::Center)
                .push(last_year_slider)
                .push(last_year),
        );

        let game = Container::new(
            main_screen
                .align_items(Alignment::Center)
                .push(reset_button)
                .push(secondary_screen)
                .push(result),
        );

        let content = match self.screen {
            Screen::Game => column![menu, container_slider, game].align_items(Alignment::Center),
            Screen::TrainingStep1 => column![menu, game].align_items(Alignment::Center),
        };

        Container::new(content)
            .width(Length::Fill)
            //.height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
