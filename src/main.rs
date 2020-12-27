use chrono::prelude::*;

fn main() {
    println!("START");
    let calendar = NaiveDate::from_ymd(1584, 1, 1).iter_days();
    for dt in calendar {
        if !(shakuntala_devi_trainer::shakuntala_devi(dt) == dt.weekday()) {
            println!(
                "{:#?} find {} but should be {}",
                dt,
                shakuntala_devi_trainer::shakuntala_devi(dt),
                dt.weekday()
            );
            break;
        }
        if dt.year() == 10000 {
            break;
        };
    }
}
