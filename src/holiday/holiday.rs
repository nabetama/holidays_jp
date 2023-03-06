use anyhow::Result;

use chrono::{Local, NaiveDate};

use crate::CliOption;

use super::dates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_holiday() -> Result<()> {
        let opt = CliOption {
            date: "20230101".to_string(),
            gen: false,
            date_format: "%Y%m%d".to_string(),
        };

        let (ok, holiday) = get_holiday(&opt);

        assert!(ok);
        assert_eq!(holiday, "元日");

        Ok(())
    }

    #[test]
    fn test_get_holiday_is_not_holiday() -> Result<()> {
        let opt = CliOption {
            date: "2023/02/02".to_string(),
            gen: false,
            date_format: "%Y/%m/%d".to_string(),
        };

        let (ok, holiday) = get_holiday(&opt);

        assert!(!ok);
        assert_eq!(holiday, "");

        Ok(())
    }
}

pub fn get_holiday(opt: &CliOption) -> (bool, &'static str) {
    let dt: String = if opt.date.is_empty() {
        Local::now().format(&opt.date_format).to_string()
    } else {
        NaiveDate::parse_from_str(&opt.date, &opt.date_format)
            .unwrap()
            .to_string()
    };

    let holidays = dates::dates();
    let name = holidays.get(&dt.as_str());

    match name {
        Some(name) => (true, name),
        None => (false, ""),
    }
}
