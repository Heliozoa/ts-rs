use chrono::{
    Date, DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
    Utc,
};

use super::TS;
use crate::Dependency;

crate::impl_primitives! {
    NaiveDateTime, NaiveDate, NaiveTime, Utc, Local, FixedOffset => "Date",
    Duration => "string"
}

impl<T: TimeZone + 'static> TS for DateTime<T> {
    fn name() -> String {
        "Date".to_owned()
    }

    fn inline() -> String {
        "Date".to_owned()
    }

    fn dependencies() -> Vec<Dependency> {
        vec![]
    }

    fn transparent() -> bool {
        false
    }
}

impl<T: TimeZone + 'static> TS for Date<T> {
    fn name() -> String {
        "Date".to_owned()
    }

    fn inline() -> String {
        "Date".to_owned()
    }

    fn dependencies() -> Vec<Dependency> {
        vec![]
    }

    fn transparent() -> bool {
        false
    }
}
