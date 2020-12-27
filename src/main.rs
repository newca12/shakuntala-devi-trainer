use chrono::prelude::*;
use num_traits::cast::FromPrimitive;
use std::io;
use std::time::Instant;

fn main() {
    let random_date = shakuntala_devi_trainer::random_date();
    //let random_date = NaiveDate::from_ymd(2017, 12, 17);
    let (shakuntala_devi_answer, tips) = shakuntala_devi_trainer::shakuntala_devi(random_date);
    let mut tips = tips.iter();
    if shakuntala_devi_answer != random_date.weekday() {
        println!("Shakuntala Devi cannot found the day of {:#?}", random_date);
        std::process::exit(-1)
    }
    let start = Instant::now();
    println!(
        "Shakuntala Devi found the day of {:#?} can you to ?",
        random_date
    );
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //{ if (0..7).contains(num) { num } else { continue } },
            Err(_) => continue,
        };
        println!("Your answer is {}", Weekday::from_u32(guess).unwrap());
        if guess == shakuntala_devi_answer.num_days_from_monday() {
            println!("Congratulation !");
            break;
        } else {
            match tips.next() {
                Some(tips) => println!("tips: {:#?}", tips),
                None => println!("Sorry, no more tips"),
            };
        }
    }
    let duration = start.elapsed();
    println!("Total time {:#?}", duration);
}
