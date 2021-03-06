use chrono::prelude::*;
use chrono::NaiveDate;
use iced::{
    button, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Sandbox,
    Settings, Text,
};

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (600u32, 300u32);
    ShakuntalaDeviTrainer::run(settings).unwrap();
}

//Weekday does not implement Default so we can(t derive Default)
#[derive(Debug)]
struct ShakuntalaDeviTrainer {
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
    hint: String,
}

#[derive(Debug, Clone)]
enum Message {
    GuessDay(Weekday),
    Reset,
}

fn generate_random_value() -> (NaiveDate, Weekday, Vec<String>) {
    let random_date = shakuntala_devi_trainer::random_date();
    //let random_date = NaiveDate::from_ymd(1940, 1, 23);
    let (shakuntala_devi_answer, tips) = shakuntala_devi_trainer::shakuntala_devi(random_date);
    (random_date, shakuntala_devi_answer, tips)
}

impl Sandbox for ShakuntalaDeviTrainer {
    type Message = Message;

    fn new() -> Self {
        let (random_date, shakuntala_devi_answer, _tips) = generate_random_value();
        Self {
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
            hint: "Guess the day!".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("Shakuntala Devi trainer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::GuessDay(guess_day) => {
                let result = if guess_day == self.week_day {
                    "Congratulation !"
                } else {
                    "Try again"
                };

                self.hint = result.to_string();
            }

            Message::Reset => {
                let (random_date, shakuntala_devi_answer, _tips) = generate_random_value();
                self.week_day = shakuntala_devi_answer;
                self.random_date = random_date.to_string();
                self.hint = "Guess the day!".to_string();
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
                .on_press(Message::Reset),
            )
            .padding(16);

        let monday_button = Column::new()
            .push(
                Button::new(
                    &mut self.monday,
                    Text::new("Monday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Mon)),
            )
            .padding(1);

        let tuesday_button = Column::new()
            .push(
                Button::new(
                    &mut self.tuesday,
                    Text::new("Tuesday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Tue)),
            )
            .padding(1);

        let wednesday_button = Column::new()
            .push(
                Button::new(
                    &mut self.wednesday,
                    Text::new("Wednesday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Wed)),
            )
            .padding(1);

        let thursday_button = Column::new()
            .push(
                Button::new(
                    &mut self.thursday,
                    Text::new("Thursday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Thu)),
            )
            .padding(1);

        let friday_button = Column::new()
            .push(
                Button::new(
                    &mut self.friday,
                    Text::new("Friday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Fri)),
            )
            .padding(1);

        let saturday_button = Column::new()
            .push(
                Button::new(
                    &mut self.saturday,
                    Text::new("Saturday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Sat)),
            )
            .padding(1);

        let sunday_button = Column::new()
            .push(
                Button::new(
                    &mut self.sunday,
                    Text::new("Sunday")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .padding(8)
                .on_press(Message::GuessDay(Weekday::Sun)),
            )
            .padding(1);

        let resut = Column::new()
            .push(Text::new(&self.hint).size(48))
            .padding(8);

        let random_date = Column::new()
            .push(Text::new(&self.random_date).size(48))
            .padding(8);

        let weekday = Row::new()
            .push(monday_button)
            .push(tuesday_button)
            .push(wednesday_button)
            .push(thursday_button)
            .push(friday_button)
            .push(saturday_button)
            .push(sunday_button);

        let content = Column::new()
            .push(reset_button)
            .push(random_date)
            .push(weekday)
            .push(resut)
            .align_items(Align::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
