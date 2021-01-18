use chrono::prelude::*;
use chrono::NaiveDate;
use iced::{
    button, text_input, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Sandbox, Settings, Text, TextInput,
};

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 300u32);
    ShakuntalaDeviTrainer::run(settings).unwrap();
}

#[derive(Debug, Default)]
struct ShakuntalaDeviTrainer {
    reset: button::State,
    input: text_input::State,
    input_value: String,
    random_date: String,
    week_day: u32,
    hint: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputOnSubmit,
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
            random_date: random_date.to_string(),
            week_day: shakuntala_devi_answer.num_days_from_sunday(),
            hint: "Guess the day!".to_string(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Shakuntala Devi trainer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,

            Message::InputOnSubmit => {
                if self.input_value.parse::<u32>().is_ok() {
                    let input_val: u32 = self.input_value.parse().unwrap();

                    let result = if input_val == self.week_day {
                        "Congratulation !"
                    } else {
                        "Try again"
                    };

                    self.hint = result.to_string();
                } else {
                    self.hint = "type a number...".to_string();
                }
            }

            Message::Reset => {
                let (random_date, shakuntala_devi_answer, _tips) = generate_random_value();
                self.week_day = shakuntala_devi_answer.num_days_from_sunday();
                self.input_value = String::new();
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

        let text_input = Column::new()
            .push(
                TextInput::new(
                    &mut self.input,
                    "type number (0 Sunday, 1 Monday, ...) and press enter",
                    &self.input_value,
                    Message::InputChanged,
                )
                .padding(8)
                .size(18)
                .on_submit(Message::InputOnSubmit),
            )
            .padding(16)
            .max_width(395);

        let resut = Column::new()
            .push(Text::new(&self.hint).size(48))
            .padding(8);

        let random_date = Column::new()
            .push(Text::new(&self.random_date).size(48))
            .padding(8);

        let content = Column::new()
            .push(reset_button)
            .push(random_date)
            .push(text_input)
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
