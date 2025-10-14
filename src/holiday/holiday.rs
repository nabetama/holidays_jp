use chrono::{Local, NaiveDate};
use anyhow::{Result, anyhow};

use crate::CliOption;

use super::dates;

/// Common date formats to try when parsing dates
const DATE_FORMATS: &[&str] = &[
    "%Y%m%d",        // 20230101
    "%Y-%m-%d",      // 2023-01-01
    "%Y/%m/%d",      // 2023/01/01
    "%Y年%m月%d日",   // 2023年1月1日
    "%m/%d/%Y",      // 01/01/2023
    "%d/%m/%Y",      // 01/01/2023 (European format)
    "%Y.%m.%d",      // 2023.01.01
];

/// Parse date string with multiple format attempts
fn parse_date_flexible(date_str: &str) -> Result<NaiveDate> {
    // First try the common formats
    for format in DATE_FORMATS {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }
    
    // If all common formats fail, try to parse as ISO format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(date);
    }
    
    Err(anyhow!("Invalid date format: '{}'. Please use one of these formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, MM/DD/YYYY, DD/MM/YYYY, or YYYY.MM.DD", date_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_holiday() -> Result<(), Box<dyn std::error::Error>> {
        let opt = CliOption {
            date: "20230101".to_string(),
            gen: false,
            date_format: "%Y%m%d".to_string(),
            output_format: crate::OutputFormat::Human,
        };

        let (ok, holiday) = get_holiday(&opt)?;

        assert!(ok);
        assert_eq!(holiday, "元日");

        Ok(())
    }

    #[test]
    fn test_get_holiday_is_not_holiday() -> Result<(), Box<dyn std::error::Error>> {
        let opt = CliOption {
            date: "2023/02/02".to_string(),
            gen: false,
            date_format: "%Y/%m/%d".to_string(),
            output_format: crate::OutputFormat::Human,
        };

        let (ok, holiday) = get_holiday(&opt)?;

        assert!(!ok);
        assert_eq!(holiday, "");

        Ok(())
    }

    #[test]
    fn test_flexible_date_parsing() -> Result<(), Box<dyn std::error::Error>> {
        // Test various date formats
        let test_cases = vec![
            ("2023-01-01", "元日"),
            ("2023/01/01", "元日"),
            ("2023年1月1日", "元日"),
            ("2023.01.01", "元日"),
        ];

        for (date_str, expected_holiday) in test_cases {
            let opt = CliOption {
                date: date_str.to_string(),
                gen: false,
                date_format: "%Y%m%d".to_string(), // This should be ignored due to flexible parsing
                output_format: crate::OutputFormat::Human,
            };

            let (ok, holiday) = get_holiday(&opt)?;
            assert!(ok, "Failed to parse date: {}", date_str);
            assert_eq!(holiday, expected_holiday, "Wrong holiday for date: {}", date_str);
        }

        Ok(())
    }
}

pub fn get_holiday(opt: &CliOption) -> Result<(bool, &'static str)> {
    let dt: String = if opt.date.is_empty() {
        Local::now().format(&opt.date_format).to_string()
    } else {
        // Try flexible parsing first, fallback to specified format
        match parse_date_flexible(&opt.date) {
            Ok(date) => date.format("%Y-%m-%d").to_string(),
            Err(_) => {
                // Fallback to the specified format
                NaiveDate::parse_from_str(&opt.date, &opt.date_format)
                    .map_err(|e| anyhow!("Could not parse date '{}' using format '{}'. Error: {}. Try using a different date format or check the --help for supported formats.", opt.date, opt.date_format, e))?
                    .format("%Y-%m-%d")
                    .to_string()
            }
        }
    };

    let holidays = dates::dates();
    let name = holidays.get(&dt.as_str());

    match name {
        Some(name) => Ok((true, name)),
        None => Ok((false, "")),
    }
}
