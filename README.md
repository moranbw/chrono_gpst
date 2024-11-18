# chrono_gpst
Dead simple extension for [chrono](https://docs.rs/chrono/latest/chrono/) to convert to and from GPS Time (GPST), with or without leap seconds.

GPS Time begins at the "GPS Epoch" on January 6, 1980. It is typically represented as a "week" (since GPS Epoch) and "week seconds" that have elapsed in said week.
## Usage
```rust
use chrono_gpst::{from_gpst, GpstLike};

let date_time = chrono::NaiveDate::from_ymd_opt(2005, 1, 28)
    .unwrap()
    .and_hms_opt(13, 30, 0)
    .unwrap()
    .and_utc();
let gpst_time = date_time.gpst(true).unwrap();
/***
 *  Seconds since GPS Epoch, Weeks since GPS Epoch, Seconds elapsed in week. Adjusted for leap seconds.
 *  Gpst { seconds: 790954213.0, week: 1307, week_seconds: 480613.0 }
 ***/
let date_time = from_gpst(1307, 480613.0, true).unwrap();
/***
 *  GPST is always UTC (with drift for leap seconds, so enable that flag if needed), so we return a DateTime<Utc>.
 *  2005-01-28T13:30:00Z
 ***/
```

## Acknowledgements
Adapted from PHP algorithm here: [https://www.andrews.edu/~tzs/timeconv/timealgorithm.html](https://www.andrews.edu/~tzs/timeconv/timealgorithm.html).
Leap seconds could be added in the future, in which a new version of this crate would need to be released.