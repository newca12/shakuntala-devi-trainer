use chrono::prelude::*;
use num_traits::cast::FromPrimitive;
use std::io;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    cli: bool,
}

pub fn parse_cli() -> bool {
    let opt = Opt::from_args();
    opt.cli
}

pub fn run_cli() {
    let (random_date, shakuntala_devi_answer, tips) =
        shakuntala_devi_trainer::random_date_with_tips(
            shakuntala_devi_trainer::DEFAULT_FIRST_YEAR,
            shakuntala_devi_trainer::DEFAULT_LAST_YEAR,
        );
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
        let guess = Weekday::from_u32(guess).unwrap().pred();
        println!("Your answer is {}", guess);
        if guess == shakuntala_devi_answer {
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
