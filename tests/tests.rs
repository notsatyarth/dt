#![allow(unused)]
use chrono::{Local, TimeZone};
use chrono_tz::Tz;

#[test]
#[test]
fn get_date_now() {
    let date = dt::now();
    println!("{:?}", date);
}

#[test]
fn get_now_tz_abr() {
    let tz: Tz = "Asia/Jakarta".parse().unwrap();
    let date = dt::now_in_tz("WIB");
    println!("{:?}", date);
    let date = dt::now_in_tz("IST");
    println!("{:?}", date);
    let date = dt::now_in_tz("SGT");
    println!("{:?}", date);
    let date = dt::now_in_tz("PST");
    println!("{:?}", date);
}
