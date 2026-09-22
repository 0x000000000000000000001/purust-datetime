pub fn Data_DateTime_Instant_toDateTimeImpl(
    constructor: purust_core::Func7<i64, i64, i64, i64, i64, i64, i64, std::rc::Rc<Purs_Data_DateTime::DateTime>>,
    instant: f64,
) -> std::rc::Rc<Purs_Data_DateTime::DateTime> {
    // A valid PureScript Instant fits inside the ECMAScript TimeClip range.
    assert!(instant.is_finite() && instant.abs() <= 8_640_000_000_000_000.0,
        "Data.DateTime.Instant: invalid instant");
    let milliseconds = instant.trunc() as i64;
    let [year, month, day] = Purs_Data_Date::purust_date_from_days(milliseconds.div_euclid(86_400_000));
    let time = milliseconds.rem_euclid(86_400_000);
    constructor(year, month, day, time / 3_600_000, (time / 60_000) % 60, (time / 1000) % 60, time % 1000)
}

pub fn Data_DateTime_Instant_fromDateTimeImpl() -> crate::UnknownType {
    crate::Value::Func7(purust_core::Func7::Shared(std::rc::Rc::new(
        |year, month, day, hour, minute, second, millisecond| {
            crate::mk_number(Purs_Data_Date::purust_utc_milliseconds(
                year.unwrap_int(),
                month.unwrap_int(),
                day.unwrap_int(),
                hour.unwrap_int(),
                minute.unwrap_int(),
                second.unwrap_int(),
                millisecond.unwrap_int(),
            ))
        },
    )))
}
