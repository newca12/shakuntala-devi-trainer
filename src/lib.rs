use chrono::prelude::*;
#[cfg(test)]
use chrono::Duration;
use num_traits::cast::FromPrimitive;
//use std::assert_eq;

//https://stackoverflow.com/questions/6385190/correctness-of-sakamotos-algorithm-to-find-the-day-of-week
pub fn tomohiko_sakamoto(dt: Date<Utc>) -> Weekday {
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
pub fn shakuntala_devi(dt: Date<Utc>) -> Weekday {
    use std::collections::HashMap;
    //https://stackoverflow.com/questions/725098/leap-year-calculation
    //https://en.wikipedia.org/wiki/Leap_year#Algorithm
    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0) && (y % 100 != 0) || (y % 400 == 0)
    }
    const T2: [i32; 12] = [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
    const T3: [i32; 7] = [0, 5, 3, 1, 6, 4, 2];
    let mut cycled = T3.iter().cycle();
    let mut years = HashMap::new();
    for year in (1584..1599).step_by(4) {
        years.insert(year, 7 + *cycled.next().unwrap());
    }
    let mut cycled = T3.iter().cycle();
    for year in (1600..1699).step_by(4) {
        years.insert(year, 6 + *cycled.next().unwrap());
    }
    let mut cycled = T3.iter().cycle();
    for year in (1700..1799).step_by(4) {
        years.insert(year, 4 + *cycled.next().unwrap());
    }
    let mut cycled = T3.iter().cycle();
    for year in (1800..1899).step_by(4) {
        years.insert(year, 2 + *cycled.next().unwrap());
    }
    let mut cycled = T3.iter().cycle();
    for year in (1900..10000).step_by(4) {
        years.insert(year, *cycled.next().unwrap());
    }
    let day = dt.day() % 7;
    let day = (day as i32 + T2[dt.month0() as usize]) % 7;
    let t1 = years.get(&dt.year());
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
                day + years.get(&nearest_leap_year).unwrap() + dt.year() - nearest_leap_year
            }
        }
        None => {
            let mut nearest_leap_year = dt.year() - 1;
            while !is_leap_year(nearest_leap_year) {
                nearest_leap_year -= 1;
            }
            day + years.get(&nearest_leap_year).unwrap() + dt.year() - nearest_leap_year
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
    let mut dt = Utc.ymd(1583, 1, 1);
    while dt.year() < 9999 {
        assert_eq!(tomohiko_sakamoto(dt), dt.weekday());
        dt = dt + Duration::days(1);
    }
}

#[test]
fn shakuntala_devi_check() {
    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0) && (y % 100 != 0) || (y % 400 == 0)
    }
    let mut dt = Utc.ymd(1584, 1, 1);
    while dt.year() < 2101 {
        if is_leap_year(dt.year()) {
            assert_eq!(shakuntala_devi(dt), dt.weekday())
        };
        dt = dt + Duration::days(1);
    }
}

#[test]
fn shakuntala_devi_unit_check() {
    let dt = Utc.ymd(1928, 1, 7);
    println!("response {} {} ", dt.year(), dt.weekday());
    assert_eq!(shakuntala_devi(dt), dt.weekday());
}
#[test]
fn leap_year_unit_check() {
    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0) && (y % 100 != 0) || (y % 400 == 0)
    }
    assert!(is_leap_year(1584));
}

#[test]
fn leap_year_check() {
    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0) && (y % 100 != 0) || (y % 400 == 0)
    }
    use std::collections::HashMap;
    const T3: [i32; 7] = [0, 5, 3, 1, 6, 4, 2];
    let mut cycled = T3.iter().cycle();
    let mut years = HashMap::new();
    for year in (1904..1909).step_by(4) {
        years.insert(year, *cycled.next().unwrap());
    }
    for year in years {
        println!("is leap {} {}?", year.0, year.1);
        assert!(is_leap_year(year.0))
    }
}

#[test]
fn leap_year_reverse_check() {
    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0) && (y % 100 != 0) || (y % 400 == 0)
    }
    use std::collections::HashMap;
    const T3: [i32; 7] = [0, 5, 3, 1, 6, 4, 2];
    let mut dt = Utc.ymd(1900, 1, 1);
    let mut cycled = T3.iter().cycle();
    let mut years = HashMap::new();
    for year in (1900..10000).step_by(4) {
        years.insert(year, *cycled.next().unwrap());
    }
    while dt.year() < 10000 {
        if is_leap_year(dt.year()) {
            assert!(years.get(&dt.year()) != None)
        };
        dt = dt + Duration::days(1);
    }
}
