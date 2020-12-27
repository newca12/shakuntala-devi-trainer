#[macro_use]
extern crate lazy_static;
use chrono::prelude::*;
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

lazy_static! {
    static ref YEARS: HashMap<i32, i32> = {
        const T3: [i32; 7] = [0, 5, 3, 1, 6, 4, 2];
        let mut years = HashMap::new();
        let mut cycled = T3.iter().cycle();
        for year in (1584..1600).step_by(4) {
            years.insert(year, *cycled.next().unwrap());
        }
        let mut cycled = T3.iter().cycle();
        for year in (1600..1700).step_by(4) {
            years.insert(year, 6 + *cycled.next().unwrap());
        }
        let mut cycled = T3.iter().cycle();
        for year in (1700..1800).step_by(4) {
            years.insert(year, 4 + *cycled.next().unwrap());
        }
        let mut cycled = T3.iter().cycle();
        for year in (1800..1900).step_by(4) {
            years.insert(year, 2 + *cycled.next().unwrap());
        }
        let mut cycled = T3.iter().cycle();
        for year in (1900..2000).step_by(4) {
            years.insert(year, *cycled.next().unwrap());
        }
        let mut cycled = T3.iter().cycle();
        for year in (2000..2100).step_by(4) {
            years.insert(year, 6 + *cycled.next().unwrap());
        }
        let mut cycled = T3.iter().cycle();
        for year in (2100..2200).step_by(4) {
            years.insert(year, 4 + *cycled.next().unwrap());
        }
        years
    };
}

//https://stackoverflow.com/questions/725098/leap-year-calculation
//https://en.wikipedia.org/wiki/Leap_year#Algorithm
fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0) && (y % 100 != 0) || (y % 400 == 0)
}

//https://stackoverflow.com/questions/6385190/correctness-of-sakamotos-algorithm-to-find-the-day-of-week
pub fn tomohiko_sakamoto(dt: NaiveDate) -> Weekday {
    const DAYS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if dt.month() < 3 {
        dt.year() - 1
    } else {
        dt.year()
    };
    let day = (y + y / 4 - y / 100 + y / 400 + DAYS[dt.month0() as usize] + dt.day() as i32) % 7;
    Weekday::from_i32(day).unwrap().pred()
}

//https://www.youtube.com/watch?v=4LHzUkfQ8oE&t=534s
//https://brainly.in/question/19415705
//https://fiat-knox.livejournal.com/1067226.html
pub fn shakuntala_devi(dt: NaiveDate) -> Weekday {
    const T2: [i32; 12] = [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
    let day = dt.day() % 7;
    let day = (day as i32 + T2[dt.month0() as usize]) % 7;
    let t1 = YEARS.get(&dt.year());
    let day = match t1 {
        Some(result) => {
            if is_leap_year(dt.year()) {
                if dt.month() > 2 {
                    day + result
                } else {
                    day + result - 1
                }
            } else {
                let mut nearest_leap_year = dt.year() - 1;
                while !is_leap_year(nearest_leap_year) {
                    nearest_leap_year -= 1;
                }
                day + YEARS.get(&nearest_leap_year).unwrap() + dt.year() - nearest_leap_year
            }
        }
        None => {
            let mut nearest_leap_year = dt.year() - 1;
            while !is_leap_year(nearest_leap_year) {
                nearest_leap_year -= 1;
            }
            day + YEARS.get(&nearest_leap_year).unwrap() + dt.year() - nearest_leap_year
        }
    };

    match Weekday::from_i32(day.rem_euclid(7)) {
        Some(d) => d.pred(),
        None => {
            println!("{:#?}", dt);
            Weekday::Fri
        }
    }
}

#[test]
fn tomohiko_sakamoto_check() {
    let calendar = NaiveDate::from_ymd(1583, 1, 1).iter_days();
    for dt in calendar {
        assert_eq!(tomohiko_sakamoto(dt), dt.weekday());
        if dt.year() == 10000 {
            break;
        };
    }
}

#[test]
fn shakuntala_devi_check() {
    let calendar = NaiveDate::from_ymd(1584, 1, 1).iter_days();
    for dt in calendar {
        assert_eq!(shakuntala_devi(dt), dt.weekday());
        if dt.year() == 2204 {
            break;
        };
    }
}

#[test]
fn shakuntala_devi_unit_check() {
    let dt = NaiveDate::from_ymd(1928, 1, 7);
    println!("response {} {} ", dt.year(), dt.weekday());
    assert_eq!(shakuntala_devi(dt), dt.weekday());
}
#[test]
fn leap_year_unit_check() {
    assert!(is_leap_year(1584));
}

#[test]
fn leap_year_check() {
    for year in 1853..10000 {
        assert!(is_leap_year(year) == NaiveDate::from_ymd_opt(year, 2, 29).is_some());
    }
}

#[test]
fn leap_year_reverse_check() {
    for year in 1853..2200 {
        if is_leap_year(year) {
            assert!(YEARS.get(&year) != None)
        };
    }
}
