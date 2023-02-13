use anyhow::Result;
use std::collections::HashMap;

use chrono::{Local, NaiveDate};

use crate::{reader::csv_reader::get_holidays, CliOption};

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

    #[test]
    fn test_find_matches() {
        let opt = CliOption {
            file: "assets/syukujitsu.csv".to_string(),
            date: "2023/01/01".to_string(),
        };

        match get_holidays(&opt.file) {
            Ok(holidays) => {
                let mut result = Vec::new();
                let _ = find_holiday(holidays, opt, &mut result);
                assert_eq!(
                    String::from_utf8(result).unwrap(),
                    "2023/01/01 is holiday (元日)\n"
                )
            }
            Err(err) => {
                eprintln!("{}", err.to_string())
            }
        }
    }
}

#[allow(dead_code)]
pub fn is_holiday(dt: NaiveDate) -> bool {
    let holidays = get_holidays("assets/syukujitsu.csv");
    let result = match holidays {
        Ok(holidays) => holidays.contains_key(&dt.format("%Y/%m/%d").to_string()),
        Err(_) => false,
    };
    result
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
    holidays: HashMap<String, String>,
    opt: CliOption,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
    match holidays.get(&opt.date) {
        Some(holiday) => writeln!(writer, "{} is holiday ({})", opt.date, holiday),
        None => writeln!(writer, "{} is not holiday", opt.date),
    }
}
