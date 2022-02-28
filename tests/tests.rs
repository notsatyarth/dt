#![allow(unused)]
use chrono::{Local, TimeZone};
use chrono_tz::Tz;
use dt::matcher;

#[test]
fn find_match() {
    let mut result = Vec::new();
    matcher::find_matches("hello 1", "h", &mut result);
    assert_eq!(std::str::from_utf8(&result).unwrap(), "hello 1");
}

#[test]
fn get_date_now() {
    let date = dt::now();
    println!("{:?}", date);
}

#[test]
fn get_now_tz_abr() {
    // let tz: Tz = "Asia/Jakarta".parse().unwrap();
    // let date = dt::now_in_tz("WIB");
    // assert_eq!(
    //     Local::now().with_timezone(&tz).to_rfc3339(),
    //     date.to_rfc3339()
    // );
    let date = dt::now_in_tz("IST");
    println!("{:?}", date);
    let date = dt::now_in_tz("SGT");
    println!("{:?}", date);
    let date = dt::now_in_tz("PST");
    println!("{:?}", date);
}
