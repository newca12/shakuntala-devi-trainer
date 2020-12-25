use chrono::prelude::*;
use chrono::Duration;

fn main() {
    println!("START");
    let mut dt = Utc.ymd(1584, 1, 1);
    while dt.year() < 10000 {
        if !(shakuntala_devi_trainer::shakuntala_devi(dt) == dt.weekday()) {
            println!("{:#?} find {} but should be {}",dt, shakuntala_devi_trainer::shakuntala_devi(dt), dt.weekday());
            break;
        }
        dt = dt + Duration::days(1);
    }
}