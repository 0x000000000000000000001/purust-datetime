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

pub fn date_days(year: i64, month: i64, day: i64) -> i64 {
    let year = year + (month - 1).div_euclid(12);
    let month = (month - 1).rem_euclid(12) as usize;
    year_start(year) + month_lengths(year)[..month].iter().sum::<i64>() + day - 1
}

// Milliseconds since the epoch for a UTC date-time, with the Date.UTC clamping
// of years 0..99 followed by setUTCFullYear, exactly like Date.js.
pub fn purust_utc_milliseconds(
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
    millisecond: i64,
) -> f64 {
    let utc_year = if (0..100).contains(&year) { year + 1900 } else { year };
    let mut date = purust_date_from_days(date_days(utc_year, month, day));
    if (0..100).contains(&year) {
        date = purust_date_from_days(date_days(year, date[1], date[2]));
    }
    ((date_days(date[0], date[1], date[2]) * 86_400 + hour * 3_600 + minute * 60 + second) * 1_000
        + millisecond) as f64
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

// The `createDate` part of Date.js: Date.UTC clamps years in 0..99 to
// 1900..1999, then setUTCFullYear restores the original year.
fn create_date_days(year: i64, month: i64, day: i64) -> i64 {
    let utc_year = if (0..100).contains(&year) { year + 1900 } else { year };
    let mut date = purust_date_from_days(date_days(utc_year, month, day));
    if (0..100).contains(&year) {
        date = purust_date_from_days(date_days(year, date[1], date[2]));
    }
    date_days(date[0], date[1], date[2])
}

pub fn Data_Date_calcWeekday() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Shared(Rc::new(|year, month, day| {
        let days = create_date_days(year.unwrap_int(), month.unwrap_int(), day.unwrap_int());
        // 1970-01-01 was a Thursday, so epoch day 0 is weekday 4.
        crate::mk_int((days.rem_euclid(7) + 4).rem_euclid(7))
    })))
}

pub fn Data_Date_calcDiff() -> crate::UnknownType {
    crate::Value::Func6(purust_core::Func6::Shared(Rc::new(
        |y1, m1, d1, y2, m2, d2| {
            let first = create_date_days(y1.unwrap_int(), m1.unwrap_int(), d1.unwrap_int());
            let second = create_date_days(y2.unwrap_int(), m2.unwrap_int(), d2.unwrap_int());
            crate::mk_number(((first - second) * 86_400_000) as f64)
        },
    )))
}
