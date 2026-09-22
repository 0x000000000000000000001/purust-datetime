use std::rc::Rc;

// Milliseconds since the epoch for a DateRec, using the shared Date.UTC
// normalization from Data/Date.rs.
fn record_milliseconds(rec: &crate::UnknownType) -> f64 {
    Purs_Data_Date::purust_utc_milliseconds(
        rec.get_year().unwrap_int(),
        rec.get_month().unwrap_int(),
        rec.get_day().unwrap_int(),
        rec.get_hour().unwrap_int(),
        rec.get_minute().unwrap_int(),
        rec.get_second().unwrap_int(),
        rec.get_millisecond().unwrap_int(),
    )
}

pub fn Data_DateTime_calcDiff() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(|first, second| {
        crate::mk_number(record_milliseconds(&first) - record_milliseconds(&second))
    })))
}

pub fn Data_DateTime_adjustImpl(
    just: purust_core::Func1<crate::UnknownType, Rc<Purs_Data_Maybe::Maybe>>,
    nothing: Rc<Purs_Data_Maybe::Maybe>,
    offset: f64,
    rec: crate::UnknownType,
) -> Rc<Purs_Data_Maybe::Maybe> {
    let ms = record_milliseconds(&rec) + offset;
    // The ECMAScript time value range is +/-8.64e15 ms; outside it, getTime()
    // returns NaN and the JS FFI resolves to Nothing.
    if !ms.is_finite() || ms.abs() > 8.64e15 {
        return nothing;
    }
    // TimeClip truncates toward zero.
    let ms = ms.trunc();
    let days = (ms / 86_400_000.0).floor() as i64;
    let time_ms = ms - (days as f64) * 86_400_000.0;
    let date = Purs_Data_Date::purust_date_from_days(days);
    let hour = (time_ms / 3_600_000.0).floor() as i64;
    let minute = ((time_ms - (hour as f64) * 3_600_000.0) / 60_000.0).floor() as i64;
    let second = ((time_ms - (hour as f64) * 3_600_000.0 - (minute as f64) * 60_000.0) / 1_000.0)
        .floor() as i64;
    let millisecond = (time_ms
        - (hour as f64) * 3_600_000.0
        - (minute as f64) * 60_000.0
        - (second as f64) * 1_000.0) as i64;
    let record = crate::Value::Record_day_hour_millisecond_minute_month_second_year(
        PerceusPtr::new(Record_day_hour_millisecond_minute_month_second_year {
            year: Some(crate::mk_int(date[0])),
            month: Some(crate::mk_int(date[1])),
            day: Some(crate::mk_int(date[2])),
            hour: Some(crate::mk_int(hour)),
            minute: Some(crate::mk_int(minute)),
            second: Some(crate::mk_int(second)),
            millisecond: Some(crate::mk_int(millisecond)),
        }),
    );
    just(record)
}
