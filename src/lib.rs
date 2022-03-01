#![allow(unused)]
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;

use chrono::{TimeZone, Utc};

use timezone_abbreviations::*;

pub fn now() -> DateTime<FixedOffset> {
    return Local::now().with_timezone(Local::now().offset());
}

pub fn now_in_tz(tz_str: &str) -> DateTime<FixedOffset> {
    let offset = get_offset_from_tz_str(tz_str).expect("Cannot parse timezone abbreviation");
    now().with_timezone(&offset)
}

pub fn get_offset_from_tz_str(tz_str: &str) -> Option<FixedOffset> {
    let mut offset: Option<FixedOffset> = None;
    if tz_str == "local" {
        offset = Some(*Local::now().offset())
    } else {
        let abbr = &timezone_abbreviations::timezone(tz_str).expect("Could not parse timezone")[0];
        let offset_seconds = abbr.hour_offset as i32 * 3600 + abbr.minute_offset as i32 * 60;
        if abbr.sign.is_plus() {
            offset = Some(FixedOffset::east(offset_seconds));
        } else {
            offset = Some(FixedOffset::west(offset_seconds));
        }
    }
    offset
}
