use std::num::ParseFloatError;

use time::{Date, OffsetDateTime, format_description::well_known::Iso8601, error::Parse};

pub fn parse_float(arg: &str) -> Result<f64, ParseFloatError> {
    let value = arg.trim().parse::<f64>()?;
    let value = format!("{:.2}", value).parse::<f64>()?;

    Ok(value)
}

pub fn parse_date(arg: &str) -> Result<Date, Parse> {
    Date::parse(arg, &Iso8601::DEFAULT)
}

pub fn default_date() -> Date {
    OffsetDateTime::now_local().unwrap().date()
}