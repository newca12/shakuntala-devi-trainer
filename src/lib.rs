#[macro_use]
extern crate lazy_static;
use chrono::prelude::*;
use chrono::Duration;
use num_traits::cast::FromPrimitive;
use rand::Rng;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
};

pub const MIN_YEAR: u32 = 1583;
pub const MAX_YEAR: u32 = 2204;
pub const DEFAULT_FIRST_YEAR: u32 = 1932;
pub const DEFAULT_LAST_YEAR: u32 = 2032;

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
pub fn shakuntala_devi(dt: NaiveDate) -> (Weekday, VecDeque<String>) {
    let mut v: VecDeque<String> = VecDeque::new();
    const T2: [i32; 12] = [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
    let day = dt.day() % 7;
    let day = (day as i32 + T2[dt.month0() as usize]) % 7;
    v.push_back(format!("Step 1 is {}", day));
    let t1 = YEARS.get(&dt.year());
    let day = match t1 {
        Some(result) => {
            v.push_back(format!("Step 2 is {:#?}", result));
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
                v.push_back(format!("Step 2 is {:#?}", nearest_leap_year));
                v.push_back(format!(
                    "Step 3 is {:#?}",
                    YEARS.get(&nearest_leap_year).unwrap()
                ));
                day + YEARS.get(&nearest_leap_year).unwrap() + dt.year() - nearest_leap_year
            }
        }
        None => {
            let mut nearest_leap_year = dt.year() - 1;
            while !is_leap_year(nearest_leap_year) {
                nearest_leap_year -= 1;
            }
            v.push_back(format!("Step 2 is {:#?}", nearest_leap_year));
            v.push_back(format!(
                "Step 3 is {:#?}",
                YEARS.get(&nearest_leap_year).unwrap()
            ));
            day + YEARS.get(&nearest_leap_year).unwrap() + dt.year() - nearest_leap_year
        }
    };

    (Weekday::from_i32(day.rem_euclid(7)).unwrap().pred(), v)
}

//http://mathforum.org/library/drmath/view/62324.html
//https://medium.com/explorations-in-python/calculating-the-day-of-the-week-with-zellers-congruence-in-python-8009001dd84e
pub fn zeller(dt: NaiveDate) -> Weekday {
    let mut year = dt.year();
    let mut month = dt.month();
    if dt.month() < 3 {
        month += 12;
        year -= 1;
    }
    Weekday::from_i32(
        (dt.day() as i32 - 2 + (13 * (month + 1) / 5) as i32 + year + year / 4 - year / 100
            + year / 400)
            % 7,
    )
    .unwrap()
}

#[allow(clippy::useless_conversion)]
pub fn st_mag_53(dt: NaiveDate) -> Weekday {
    let (j, m, a) = (dt.day(), dt.month(), dt.year() as u32);
    let man = (0.6 + 1.0 / f64::from(m) + 0.001) as u32;
    let mp = m + 12 * man;
    let ap = a - man;
    let jd = j
        + ((367.0 * (f64::from(mp) - 1.0) + 5.0) / 12.0 + 0.001) as u32
        + (365.25 * (f64::from(ap) + 4712.0) + 0.001) as u32;
    let jd = jd - (((f64::from(ap) / 100.0) as u32 + (f64::from(ap) / 400.0) as u32) as u32);
    let js = f64::from(jd - 1720977) / 7.0;
    let js = (7.0 * f64::from(js - f64::from(js as u32)) + 0.001) as i32;
    Weekday::from_i32(js).unwrap()
}

pub fn svm_86_distance(dt: NaiveDate) -> u32 {
    let (j, m, mut a) = (dt.day(), dt.month(), dt.year() as u32);
    let mut n = a * 365 + 31 * (m - 1) + j;
    if m <= 2 {
        a -= 1;
    }
    n = n + (a / 4) - (a / 100) + (a / 400);
    if m > 2 {
        n -= (f64::from(m - 1) * 0.4 + 2.7) as u32;
    }
    n
}

pub fn svm_86(dt: NaiveDate) -> Weekday {
    //let base_date = NaiveDate::from_ymd_opt(1901, 1, 7);
    let base_date = NaiveDate::from_ymd_opt(1583, 1, 3).unwrap();
    let distance = svm_86_distance(dt) - svm_86_distance(base_date);
    Weekday::from_u32(distance % 7).unwrap()
}

pub fn random_date(from_year: u32, to_year: u32) -> NaiveDate {
    let start = NaiveDate::from_ymd_opt(from_year.try_into().unwrap(), 1, 1)
        .unwrap()
        .num_days_from_ce();
    let end = NaiveDate::from_ymd_opt(to_year.try_into().unwrap(), 1, 1)
        .unwrap()
        .num_days_from_ce();
    let days = rand::thread_rng().gen_range(1..end - start);
    let dt = NaiveDate::from_ymd_opt(from_year.try_into().unwrap(), 1, 7).unwrap();
    dt + Duration::days(days as i64)
}

pub fn random_date_with_tips(
    from_year: u32,
    to_year: u32,
) -> (NaiveDate, Weekday, VecDeque<String>) {
    let random_date = random_date(from_year, to_year);
    //let random_date = NaiveDate::from_ymd(1940, 1, 23);
    let (shakuntala_devi_answer, tips) = shakuntala_devi(random_date);
    (random_date, shakuntala_devi_answer, tips)
}

#[test]
fn tomohiko_sakamoto_check() {
    let calendar = NaiveDate::from_ymd_opt(1583, 1, 1).unwrap().iter_days();
    for dt in calendar {
        assert_eq!(tomohiko_sakamoto(dt), dt.weekday(), "testing {}", dt);
        if dt.year() == 10000 {
            break;
        };
    }
}

#[test]
fn shakuntala_devi_unit_check() {
    let dt = NaiveDate::from_ymd_opt(1928, 1, 7).unwrap();
    println!("response {} {} ", dt.year(), dt.weekday());
    assert_eq!(shakuntala_devi(dt).0, dt.weekday(), "testing {}", dt);
}

#[test]
fn shakuntala_devi_check() {
    let calendar = NaiveDate::from_ymd_opt(1584, 1, 1).unwrap().iter_days();
    for dt in calendar {
        assert_eq!(shakuntala_devi(dt).0, dt.weekday(), "testing {}", dt);
        if dt.year() == 2204 {
            break;
        };
    }
}

#[test]
fn zeller_unit_check() {
    let dt = NaiveDate::from_ymd_opt(1928, 1, 7).unwrap();
    println!("response {} {} ", dt.year(), dt.weekday());
    assert_eq!(zeller(dt), dt.weekday(), "testing {}", dt);
}

#[test]
fn zeller_check() {
    let calendar = NaiveDate::from_ymd_opt(1584, 1, 1).unwrap().iter_days();
    for dt in calendar {
        assert_eq!(zeller(dt), dt.weekday(), "testing {}", dt);
        if dt.year() == 10000 {
            break;
        };
    }
}

#[test]
fn st_mag_53_unit_check() {
    let dt = NaiveDate::from_ymd_opt(1980, 1, 7).unwrap();
    println!("response {} {} ", dt.year(), dt.weekday());
    assert_eq!(st_mag_53(dt), dt.weekday(), "testing {}", dt);
}

#[test]
fn st_mag_53_check() {
    let calendar = NaiveDate::from_ymd_opt(1700, 1, 1).unwrap().iter_days();
    for dt in calendar {
        assert_eq!(st_mag_53(dt), dt.weekday(), "testing {}", dt);
        if dt.year() == 2000 {
            break;
        };
    }
}

#[test]
fn svm_86_check() {
    let calendar = NaiveDate::from_ymd_opt(1583, 1, 3).unwrap().iter_days();
    for dt in calendar {
        assert_eq!(svm_86(dt), dt.weekday(), "testing {}", dt);
        if dt.year() == 10000 {
            break;
        };
    }
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
