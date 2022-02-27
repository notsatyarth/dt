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
    let date = dt::now_from_tz_abbrev("WIB");
    println!("{:?}", date);
    let date = dt::now_from_tz_abbrev("IST");
    println!("{:?}", date);
    let date = dt::now_from_tz_abbrev("SGT");
    println!("{:?}", date);
    let date = dt::now_from_tz_abbrev("PST");
    println!("{:?}", date);
}
