#![allow(unused)]
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono_tz::Tz;
use timezone_abbreviations::*;

pub mod matcher {
    pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
        for line in content.lines() {
            if line.contains(pattern) {
                write!(writer, "{}", line);
            }
        }
    }
}

pub fn now() -> DateTime<Local> {
    return Local::now();
}

pub fn now_from_tz_abbrev(tz_str: &str) -> DateTime<FixedOffset> {
    let abbr = &timezone_abbreviations::timezone(tz_str).expect("Could not parse")[0];
    let mut offset: Option<FixedOffset> = None;
    let offset_seconds = abbr.hour_offset as i32 * 3600 + abbr.minute_offset as i32 * 60;
    if abbr.sign.is_plus() {
        offset = Some(FixedOffset::east(offset_seconds));
    } else {
        offset = Some(FixedOffset::west(offset_seconds));
    }
    return now().with_timezone(&offset.unwrap());
}
