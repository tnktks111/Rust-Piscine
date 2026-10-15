use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        return None;
    }

    let year = i32::try_from(year).ok()?;
    let date = NaiveDate::from_ymd_opt(year, 7, 2)?;
    Some(date.weekday())
}
