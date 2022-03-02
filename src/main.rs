use chrono::NaiveDateTime;
#[allow(unused)]
use chrono::{format::StrftimeItems, DateTime, FixedOffset, Offset, TimeZone};
use clap::Parser;

/// Transform dates to different timezones using timezone abbreviations
#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
struct Cli {
    /// The time string to parse
    #[clap(short, long, default_value = "now")]
    date: String,
    /// The timezone abbreviation to convert to
    #[clap(short, long, default_value = "local")]
    to: String,

    /// The timezone abbreviation to convert from
    #[clap(short, long, default_value = "local")]
    from: String,
}

fn main() {
    let args = Cli::parse();
    let output: Option<DateTime<FixedOffset>>;
    if args.date == "now" {
        if args.to == "local" {
            output = Some(dt::now());
        } else {
            output = Some(dt::now_in_tz(&args.to.to_uppercase()));
        }
    } else {
        output = Some(handle_timestamp(&args.date, &args.from, &args.to));
    }
    match output {
        Some(time) => println!(
            "{}",
            time.to_rfc3339_opts(chrono::SecondsFormat::Millis, false)
        ),
        None => println!("Nothing"),
    }
}

fn handle_timestamp(date_str: &str, from_tz_str: &str, to_tz_str: &str) -> DateTime<FixedOffset> {
    let date = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
        .expect("Incorrect format , use YY-MM-DD HH:MM:SS");
    let from_tz = dt::get_offset_from_tz_str(from_tz_str).expect("Invalid from timezone");
    let to_tz = dt::get_offset_from_tz_str(to_tz_str).expect("Invalid to timezone");
    let date_in_local = from_tz.from_local_datetime(&date).unwrap();
    date_in_local.with_timezone(&to_tz)
}
