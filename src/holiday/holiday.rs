use anyhow::Result;
use std::collections::HashMap;

use chrono::{Local, NaiveDate};

use crate::CliOption;

use super::dates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_holiday() {
        let dt = NaiveDate::parse_from_str("2023/01/01", "%Y/%m/%d");
        match dt {
            Ok(dt) => assert_eq!(is_holiday(dt), true),
            Err(err) => eprintln!("{:?}", err),
        }
    }

    #[test]
    fn test_get_date() -> Result<()> {
        let dt = get_date("2023/01/01")?;
        assert_eq!(dt, "2023/01/01");

        Ok(())
    }
}

#[allow(dead_code)]
pub fn is_holiday(dt: NaiveDate) -> bool {
    let holidays = dates::dates();
    holidays.contains_key(dt.format("%Y/%m/%d").to_string().as_str())
}

pub fn get_date(date_arg: &str) -> Result<String> {
    if date_arg.to_string().len() > 0 {
        match NaiveDate::parse_from_str(date_arg, "%Y/%m/%d") {
            Ok(dt) => {
                return Ok(dt.format("%Y/%m/%d").to_string());
            }
            Err(err) => return Err(err.into()),
        }
    }
    Ok(Local::now().format("%Y/%m/%d").to_string())
}

pub fn find_holiday(
    holidays: HashMap<&str, &str>,
    opt: CliOption,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
    match holidays.get(opt.date.as_str()) {
        Some(holiday) => writeln!(writer, "{} is holiday ({})", opt.date, holiday),
        None => writeln!(writer, "{} is not holiday", opt.date),
    }
}
