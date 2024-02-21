#![allow(deprecated)]
#![cfg(feature = "chrono-impl")]

use chrono::{
    DateTime, Duration, FixedOffset, Local, Month, NaiveDate, NaiveDateTime, NaiveTime, Utc,
    Weekday,
};
use ts_rs::TS;

#[test]
fn chrono() {
    #[derive(TS)]
    #[allow(dead_code)]
    struct Chrono {
        date: NaiveDate,
        time: NaiveTime,
        date_time: (
            NaiveDateTime,
            DateTime<Utc>,
            DateTime<Local>,
            DateTime<FixedOffset>,
        ),
        duration: Duration,
        month: Month,
        weekday: Weekday,
    }

    assert_eq!(
        Chrono::decl(),
        "type Chrono = { date: string, time: string, date_time: [string, string, string, string], duration: string, month: string, weekday: string, }"
    )
}
