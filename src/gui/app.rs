use chrono::prelude::*;
use enum_map::EnumMap;
use num_traits::cast::FromPrimitive;

use iced::{
    alignment,
    widget::{button, column, row, text, Container, Slider},
    Alignment, Element, Length, Task,
};

use shakuntala_devi_trainer::{shakuntala_devi_nearest_leap_year, Tips, T2, YEARS};

use crate::gui::common::Screen;

use super::style::{button_day, button_start, slider_style};

//Weekday does not implement Default so we can't derive Default
#[derive(Debug, Clone)]
pub(crate) struct ShakuntalaDeviTrainer {
    screen: Screen,
    first_year: u32,
    last_year: u32,
    random_date: NaiveDate,
    week_day: Weekday,
    game_answers: [bool; 7],
    month_table_answers: [bool; 7],
    year_table_answers: [bool; 13],
    tips: Tips,
    hint: EnumMap<Screen, String>,
    start: web_time::Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    GuessDay(Weekday),
    GuessMonthTable(i32),
    GuessYearTable(i32),
    Reset,
    FirstYear(u32),
    LastYear(u32),
    GameMode,
    TrainingMonthTableMode,
    TrainingYearTableMode,
    SolutionMode,
}

impl ShakuntalaDeviTrainer {
    pub fn new() -> (ShakuntalaDeviTrainer, Task<Message>) {
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
                game_answers: [false; 7],
                month_table_answers: [false; 7],
                year_table_answers: [false; 13],
                tips,
                hint: enum_map! {
                    Screen::Game => "Guess the day!".to_string(),
                    Screen::Solution => "".to_string(),
                    Screen::TrainingMonthTable => "Which entry is the good one ?".to_string(),
                    Screen::TrainingYearTable => "Which entry is the good one ?".to_string(),
                },
                start: web_time::Instant::now(),
            },
            iced::Task::none(),
        )
    }

    pub fn title(&self) -> String {
        format!(
            "{} {}",
            "Shakuntala Devi trainer",
            env!("CARGO_PKG_VERSION")
        )
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::GuessDay(guess_day) => {
                self.game_answers[usize::try_from(guess_day.number_from_monday() - 1)
                    .ok()
                    .unwrap()] = true;
                let tries = self.game_answers.iter().filter(|&n| *n).count();
                let result = if guess_day == self.week_day {
                    self.game_answers = [true; 7];
                    format!(
                        "Congratulation ! You found {} after {} guess in {:#?}s",
                        guess_day,
                        tries,
                        self.start.elapsed().as_secs()
                    )
                } else {
                    match self.tips.0.get(tries - 1) {
                        Some(tips) => format!("tips: {:#?}", tips),
                        None => "Sorry, no more tips".to_string(),
                    }
                };
                self.hint[self.screen] = result;
            }

            Message::Reset => {
                let (random_date, shakuntala_devi_answer, tips) =
                    shakuntala_devi_trainer::random_date_with_tips(self.first_year, self.last_year);
                self.week_day = shakuntala_devi_answer;
                self.random_date = random_date;
                self.tips = tips;
                self.hint = enum_map! {
                    Screen::Game => "Guess the day!".to_string(),
                    Screen::Solution => "".to_string(),
                    Screen::TrainingMonthTable => "Which entry is the good one ?".to_string(),
                    Screen::TrainingYearTable => "Which entry is the good one ?".to_string(),
                };
                self.game_answers = [false; 7];
                self.month_table_answers = [false; 7];
                self.year_table_answers = [false; 13];
                self.start = web_time::Instant::now();
            }

            Message::TrainingMonthTableMode => {
                self.screen = Screen::TrainingMonthTable;
            }

            Message::TrainingYearTableMode => {
                self.screen = Screen::TrainingYearTable;
            }

            Message::GameMode => {
                self.screen = Screen::Game;
            }

            Message::SolutionMode => {
                self.screen = Screen::Solution;
            }

            Message::GuessMonthTable(guess) => {
                self.month_table_answers[usize::try_from(guess).ok().unwrap()] = true;
                if T2[self.random_date.month0() as usize] == guess {
                    self.month_table_answers = [true; 7];
                    self.hint[self.screen] =
                        format!("Congratulation ! {} is the right answer", guess)
                } else {
                    self.hint[self.screen] = "Try again".to_string()
                };
            }

            Message::GuessYearTable(guess) => {
                self.year_table_answers[usize::try_from(guess).ok().unwrap()] = true;
                let versatile_answer =
                    shakuntala_devi_nearest_leap_year(self.random_date.year(), &mut None);
                let answer = if versatile_answer > 12 {
                    YEARS.get(&versatile_answer).unwrap()
                } else {
                    &versatile_answer
                };
                if *answer == guess {
                    self.year_table_answers = [true; 13];
                    self.hint[self.screen] =
                        format!("Congratulation ! {} is the right answer", guess)
                } else {
                    self.hint[self.screen] = if versatile_answer > 12 {
                        format!(
                            "Try again. Tips: no direct year table entry, nearest leap year {}",
                            versatile_answer
                        )
                    } else {
                        "Try again, this is a direct year table entry".to_string()
                    }
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
        iced::Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let reset_button = column![button(
            text("Start new game")
                .align_x(alignment::Horizontal::Center)
                .size(14),
        )
        .style(button_start)
        .padding(8)
        .on_press(Message::Reset)]
        .padding(16);

        let menu_game = column![button(
            text("DAY TRAINING MODE")
                .align_x(alignment::Horizontal::Center)
                .size(14),
        )
        .padding(8)
        .on_press(Message::GameMode)
        .style(|theme, status| if self.screen == Screen::Game {
            super::style::button_menu(theme, status)
        } else {
            super::style::button_menu_inactive(theme, status)
        }),]
        .padding(16);

        let menu_month_table = column![button(
            text("MONTH TABLE")
                .align_x(alignment::Horizontal::Center)
                .size(14),
        )
        .padding(8)
        .on_press(Message::TrainingMonthTableMode)
        .style(
            |theme, status| if self.screen == Screen::TrainingMonthTable {
                super::style::button_menu(theme, status)
            } else {
                super::style::button_menu_inactive(theme, status)
            }
        ),]
        .padding(16);

        let menu_year_table = column![button(
            text("YEAR TABLE")
                .align_x(alignment::Horizontal::Center)
                .size(14),
        )
        .padding(8)
        .on_press(Message::TrainingYearTableMode)
        .style(
            |theme, status| if self.screen == Screen::TrainingYearTable {
                super::style::button_menu(theme, status)
            } else {
                super::style::button_menu_inactive(theme, status)
            }
        ),]
        .padding(16);

        let menu_solution = column![button(
            text("SOLUTION")
                .align_x(alignment::Horizontal::Center)
                .size(14),
        )
        .padding(8)
        .on_press(Message::SolutionMode)
        .style(|theme, status| if self.screen == Screen::Solution {
            super::style::button_menu(theme, status)
        } else {
            super::style::button_menu_inactive(theme, status)
        }),]
        .padding(16);

        let menu = row![menu_game, menu_solution, menu_month_table, menu_year_table];

        let column_weekday = |label, weekday, already_pressed| {
            column![if already_pressed {
                button(text(label).align_x(alignment::Horizontal::Center).size(14))
                    .padding(8)
                    .style(button_day)
            } else {
                button(text(label).align_x(alignment::Horizontal::Center).size(14))
                    .padding(8)
                    .on_press(Message::GuessDay(weekday))
                    .style(button_day)
            }]
            .padding(1)
        };

        let column_t2 = |label, weekday, already_pressed| {
            column![if already_pressed {
                button(text(label).align_x(alignment::Horizontal::Center).size(14))
                    .padding(8)
                    .style(button_day)
            } else {
                button(text(label).align_x(alignment::Horizontal::Center).size(14))
                    .padding(8)
                    .on_press(Message::GuessMonthTable(weekday))
                    .style(button_day)
            }]
            .padding(1)
        };

        let column_t3 = |label, weekday, already_pressed| {
            column![if already_pressed {
                button(text(label).align_x(alignment::Horizontal::Center).size(14))
                    .padding(8)
                    .style(button_day)
            } else {
                button(text(label).align_x(alignment::Horizontal::Center).size(14))
                    .padding(8)
                    .on_press(Message::GuessYearTable(weekday))
                    .style(button_day)
            }]
            .padding(1)
        };

        let result = column![text(&self.hint[self.screen]).size(20)].padding(8);

        let random_date = {
            let month = Month::from_u32(self.random_date.month()).unwrap().name();
            let year = self.random_date.year();
            let date = format!("{} {} {}", self.random_date.day(), month, year);
            match self.screen {
                Screen::Game => column![text(date).size(40)].padding(8),
                Screen::TrainingMonthTable => column![text(month).size(40)].padding(8),
                Screen::TrainingYearTable => column![text(year).size(40)].padding(8),
                Screen::Solution => column![text(date).size(40)].padding(8),
            }
        };

        let first_year = column![text(self.first_year.to_string()).size(10)];

        let last_year = column![text(self.last_year.to_string()).size(10)];

        let first_year_slider = column![Slider::new(
            shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
            self.first_year,
            Message::FirstYear,
        )
        .style(slider_style)]
        .padding(0)
        .spacing(3);

        let last_year_slider = column![Slider::new(
            shakuntala_devi_trainer::MIN_YEAR..=shakuntala_devi_trainer::MAX_YEAR,
            self.last_year,
            Message::LastYear,
        )
        .style(slider_style)]
        .padding(0);

        let weekday = row![
            column_weekday("Monday", Weekday::Mon, self.game_answers[0],),
            column_weekday("Tuesday", Weekday::Tue, self.game_answers[1],),
            column_weekday("Wednesday", Weekday::Wed, self.game_answers[2],),
            column_weekday("Thursday", Weekday::Thu, self.game_answers[3],),
            column_weekday("Friday", Weekday::Fri, self.game_answers[4],),
            column_weekday("Saturday", Weekday::Sat, self.game_answers[5],),
            column_weekday("Sunday", Weekday::Sun, self.game_answers[6],)
        ];

        let t3 = row![
            column_t2("0", 0, self.month_table_answers[0],),
            column_t2("1", 1, self.month_table_answers[1],),
            column_t2("2", 2, self.month_table_answers[2],),
            column_t2("3", 3, self.month_table_answers[3],),
            column_t2("4", 4, self.month_table_answers[4],),
            column_t2("5", 5, self.month_table_answers[5],),
            column_t2("6", 6, self.month_table_answers[6],),
        ];

        let t3_year = row![
            column_t3("0", 0, self.year_table_answers[0],),
            column_t3("1", 1, self.year_table_answers[1],),
            column_t3("2", 2, self.year_table_answers[2],),
            column_t3("3", 3, self.year_table_answers[3],),
            column_t3("4", 4, self.year_table_answers[4],),
            column_t3("5", 5, self.year_table_answers[5],),
            column_t3("6", 6, self.year_table_answers[6],),
        ];

        let solution = row![text(format!("{}", self.tips))];

        let (main_screen, secondary_screen) = match self.screen {
            Screen::Game => (random_date, weekday),
            Screen::TrainingMonthTable => (random_date, t3),
            Screen::TrainingYearTable => (random_date, t3_year),
            Screen::Solution => (random_date, solution),
        };

        let container_slider = Container::new(
            first_year_slider
                .push(first_year)
                .align_x(Alignment::Center)
                .push(last_year_slider)
                .push(last_year),
        );

        let game = Container::new(
            main_screen
                .align_x(Alignment::Center)
                .push(reset_button)
                .push(secondary_screen)
                .push(result),
        );

        let content = match self.screen {
            Screen::Game => column![menu, container_slider, game].align_x(Alignment::Center),
            Screen::TrainingMonthTable => column![menu, game].align_x(Alignment::Center),
            Screen::TrainingYearTable => column![menu, game].align_x(Alignment::Center),
            Screen::Solution => column![menu, game].align_x(Alignment::Center),
        }
        .spacing(3);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .style(crate::gui::style::main_container)
            .into()
    }
}
