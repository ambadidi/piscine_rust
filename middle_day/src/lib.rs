use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd;
pub fn middle_day(year: i32) -> Option<wd> {
    let nbr_of_days = if NaiveDate::from_ymd_opt(year, 2, 29).is_some() {366} else {365};
    if nbr_of_days == 366 {None}
    else {
        NaiveDate::from_ymd_opt(year, 7, 2).map(|date| date.weekday())
    }
}