use std::num::ParseFloatError;

use time::{Date, OffsetDateTime, format_description::well_known::Iso8601};

pub fn parse_float(arg: &str) -> Result<f64, ParseFloatError> {
    let value = arg.trim().parse::<f64>()?;
    let value = format!("{:.2}", value).parse::<f64>()?;

    Ok(value)
}

pub fn parse_date(arg: &str) -> Result<Date, time::error::Parse> {
    Date::parse(arg, &Iso8601::DEFAULT)
}

/// Returns the current date
pub fn default_date() -> Date {
    OffsetDateTime::now_local().unwrap().date()
}