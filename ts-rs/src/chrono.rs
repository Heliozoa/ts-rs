// we want to implement TS for deprecated types as well
#![allow(deprecated)]

use chrono::{
    DateTime, Duration, FixedOffset, Local, Month, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
    Utc, Weekday,
};

use super::TS;
use crate::Dependency;

crate::impl_primitives! {
    NaiveDateTime, NaiveDate, NaiveTime, Month, Weekday, Duration => "string",
    Utc, Local, FixedOffset => "Date"
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
