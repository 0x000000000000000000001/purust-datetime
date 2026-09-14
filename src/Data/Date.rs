use std::rc::Rc;

fn leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
}

fn year_start(year: i64) -> i64 {
    let previous = year - 1;
    let leaps = previous.div_euclid(4) - previous.div_euclid(100) + previous.div_euclid(400);
    // There are 477 Gregorian leap days before 1970 relative to year 1.
    365 * (year - 1970) + leaps - 477
}

fn month_lengths(year: i64) -> [i64; 12] {
    [31, if leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

pub fn purust_date_from_days(days: i64) -> [i64; 3] {
    assert!(days.abs() <= 100_000_000, "Date exceeds the ECMAScript date range");
    // These bounds enclose the complete ECMAScript range, including BCE years.
    let (mut low, mut high) = (-300_000, 300_001);
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if year_start(middle) <= days { low = middle; } else { high = middle; }
    }
    let mut day = days - year_start(low);
    let lengths = month_lengths(low);
    let mut month = 0;
    while day >= lengths[month] { day -= lengths[month]; month += 1; }
    [low, month as i64 + 1, day + 1]
}

fn date_days(year: i64, month: i64, day: i64) -> i64 {
    let year = year + (month - 1).div_euclid(12);
    let month = (month - 1).rem_euclid(12) as usize;
    year_start(year) + month_lengths(year)[..month].iter().sum::<i64>() + day - 1
}

pub fn Data_Date_canonicalDateImpl() -> crate::UnknownType {
    crate::Value::Func4(purust_core::Func4::Shared(Rc::new(|constructor, year, month, day| {
        let year = year.unwrap_int();
        let month = month.unwrap_int();
        let day = day.unwrap_int();
        // Reproduce the original Date.UTC call followed by setUTCFullYear.
        // In particular, 1900's non-leap February is normalized before year 0
        // is restored; changing that order would differ from the JS FFI.
        let utc_year = if (0..100).contains(&year) { year + 1900 } else { year };
        let mut date = purust_date_from_days(date_days(utc_year, month, day));
        if (0..100).contains(&year) {
            date = purust_date_from_days(date_days(year, date[1], date[2]));
        }
        constructor.unwrap_func3()(crate::mk_int(date[0]), crate::mk_int(date[1]), crate::mk_int(date[2]))
    })))
}
