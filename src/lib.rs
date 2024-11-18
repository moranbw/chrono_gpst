/*!
Dead simple extension for [chrono](https://docs.rs/chrono/latest/chrono/) to convert to and from GPS Standard Time, with or without
leap seconds. 

GPS Standard time began at the "GPS Epoch" on January 6, 1980. It is typically represented as a "week" (since GPS Epoch)
and "week seconds" that have elapsed in said week.
## Usage
```
use chrono_gpst::{from_gpst, GpstLike};

let date_time = chrono::NaiveDate::from_ymd_opt(2005, 1, 28)
    .unwrap()
    .and_hms_opt(13, 30, 0)
    .unwrap()
    .and_utc();
let gpst_time = date_time.gpst(true).unwrap();
/***
 *  Seconds since GPS Epoch, Weeks since GPS Epoch, Seconds elapsed in week. Adjusted for leap seconds.
 *  Gpst { seconds: 790954213, week: 1307, week_seconds: 480613 }
 ***/
let date_time = from_gpst(1307, 480613, true).unwrap();
/***
 *  GPST is always UTC (with drift for leap seconds, so enable that flag if needed), so we return a DateTime<Utc>.
 *  2005-01-28T13:30:00Z
 ***/
```

## Acknowledgements
Adapted from PHP algorithm here: [https://www.andrews.edu/~tzs/timeconv/timealgorithm.html](https://www.andrews.edu/~tzs/timeconv/timealgorithm.html).
Leap seconds could be added in the future, in which a new version of this crate would need to be replaced.
*/

use chrono::{DateTime, Utc};
use thiserror::Error;

/// Custom errors
#[derive(Error, Debug)]
pub enum GpstError {
    /// Error converting GPST weeks and week seconds to DateTime<Utc>.
    #[error("Error converting to date from GPST, this is unexpected.")]
    FromGpst,
    /// Error caused when provided date is earlier than GPS Epoch.
    #[error("Invalid date for GPST, is earlier than GPS Epoch: {0}")]
    BeforeGPSEpoch(String),
}

/// "GPS Epoch": 01-06-1980 00:00:00
pub const GPS_EPOCH: i64 = 315964800;
const SECONDS_PER_WEEK: f64 = 604800.0;

/// GPST data
#[derive(Debug, PartialEq)]
pub struct Gpst {
    /// Seconds since GPS Epoch
    seconds: i64,
    /// Weeks since GPS Epoch
    week: i64,
    /// Seconds in current week
    week_seconds: i64,
}

//Trait that extends [`chrono::DateTime`] / [`chrono::Utc`] for GPS Standard Time (GPST).
pub trait GpstLike {
    /// Convert to GPS Standard Time (GPST) from DateTime<UTC>. Optionally, adjust for leap seconds.
    fn gpst(&self, leap_seconds: bool) -> Result<Gpst, GpstError>;
}

impl GpstLike for DateTime<Utc> {
    fn gpst(&self, leap_seconds: bool) -> Result<Gpst, GpstError> {
        let mut seconds = self.timestamp() - GPS_EPOCH;
        if leap_seconds {
            seconds += num_leaps(seconds);
        }
        if seconds < 0 {
            GpstError::BeforeGPSEpoch(self.to_rfc3339());
        }
        let week = seconds as f64 / SECONDS_PER_WEEK;
        let week_remainder = week % 1.0;
        Ok(Gpst {
            seconds,
            week: week as i64,
            week_seconds: (week_remainder * SECONDS_PER_WEEK) as i64,
        })
    }
}

/// Given seconds since GPS Epoch, convert to a DateTime<Utc>. Optionally, adjust for leap seconds.
pub fn from_gpst_seconds(mut seconds: i64, leap_seconds: bool) -> Result<DateTime<Utc>, GpstError> {
    if leap_seconds {
        seconds -= num_leaps(seconds);
    }
    let date_time = DateTime::from_timestamp(seconds + GPS_EPOCH, 0).ok_or(GpstError::FromGpst)?;
    Ok(date_time)
}

/// Given weeks since GPS Epoch and week seconds, convert to a DateTime<Utc>. Optionally, adjust for leap seconds.
pub fn from_gpst(
    week: i64,
    week_seconds: i64,
    leap_seconds: bool,
) -> Result<DateTime<Utc>, GpstError> {
    let gps_seconds = (week * SECONDS_PER_WEEK as i64) + week_seconds;
    from_gpst_seconds(gps_seconds, leap_seconds)
}

/// Leap seconds since GPS Epoch.
const LEAP_SECONDS: [i64; 18] = [
    46828800, 78364801, 109900802, 173059203, 252028804, 315187205, 346723206, 393984007,
    425520008, 457056009, 504489610, 551750411, 599184012, 820108813, 914803214, 1025136015,
    1119744016, 1167264017,
];

/// Count how many leap seconds have occured since a given GPS timestamp.
fn num_leaps(gps_seconds: i64) -> i64 {
    let mut count = 0;
    for leap_second in LEAP_SECONDS {
        if leap_second <= gps_seconds {
            count += 1;
        }
    }
    count
}

mod tests {
    use crate::{from_gpst, Gpst, GpstLike};
    use chrono::NaiveDate;

    #[test]
    fn to() {
        let date_time = NaiveDate::from_ymd_opt(2005, 1, 28)
            .unwrap()
            .and_hms_opt(13, 30, 0)
            .unwrap()
            .and_utc();
        assert_eq!(date_time.gpst(true).unwrap(), Gpst {seconds: 790954213, week: 1307, week_seconds: 480613});
    }
    #[test]
    fn from() {
        let date_time = NaiveDate::from_ymd_opt(2005, 1, 28)
            .unwrap()
            .and_hms_opt(13, 30, 0)
            .unwrap()
            .and_utc();
        println!("{:?}", from_gpst(1307, 480613, true).unwrap());
        assert_eq!(from_gpst(1307, 480613, true).unwrap(), date_time)
    }
}
