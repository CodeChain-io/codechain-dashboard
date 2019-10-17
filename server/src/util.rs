use chrono::{DateTime, Timelike, Utc};
use std::error;
use std::fmt::Debug;
use std::result::Result;

pub fn log_error<T>(context: T, result: Result<(), Box<dyn error::Error>>)
where
    T: Debug, {
    if let Err(err) = result {
        cerror!("Error at {:?} : {}", context, err);
    }
}

pub fn floor_to_5min(time: &DateTime<Utc>) -> DateTime<Utc> {
    time.with_minute(time.minute() - (time.minute() % 5)).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap()
}

pub fn start_of_hour(time: &DateTime<Utc>) -> DateTime<Utc> {
    time.with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap()
}

pub fn start_of_day(time: &DateTime<Utc>) -> DateTime<Utc> {
    time.with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_5min() {
        fn utc_from_string(str_time: &'static str) -> DateTime<Utc> {
            DateTime::parse_from_rfc3339(str_time).unwrap().with_timezone(&Utc)
        }

        let time_a = utc_from_string("1996-12-19T16:39:57Z");
        assert_eq!(floor_to_5min(&time_a), utc_from_string("1996-12-19T16:35:00Z"));

        let time_b = utc_from_string("2018-05-22T16:00:00Z");
        assert_eq!(floor_to_5min(&time_b), utc_from_string("2018-05-22T16:00:00Z"));
    }
}
