use anyhow::Result;

use chrono::{Local, NaiveDate};

use super::dates;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_date() -> Result<()> {
        let dt = get_date("2023/01/01")?;
        assert_eq!(dt, NaiveDate::parse_from_str("20230101", "%Y%m%d")?);

        Ok(())
    }

    #[test]
    fn test_get_holiday() -> Result<()> {
        let dt = NaiveDate::parse_from_str("20220101", "%Y%m%d")?;
        let (ok, holiday) = get_holiday(dt);

        assert_eq!(ok, true);
        assert_eq!(holiday, "元日");

        Ok(())
    }
}

pub fn get_date(date_arg: &str) -> Result<NaiveDate> {
    if date_arg.to_string().len() > 0 {
        match NaiveDate::parse_from_str(date_arg, "%Y/%m/%d") {
            Ok(dt) => {
                return Ok(dt);
            }
            Err(err) => return Err(err.into()),
        }
    }
    Ok(Local::now().date_naive())
}

pub fn get_holiday(dt: NaiveDate) -> (bool, &'static str) {
    let holidays = dates::dates();
    let name = holidays.get(dt.format("%Y/%m/%d").to_string().as_str());

    match name {
        Some(name) => return (true, name),
        None => return (false, ""),
    }
}
