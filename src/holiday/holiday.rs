use chrono::{Local, NaiveDate};
use anyhow::{Result, anyhow, Context};

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
                date_format: "%Y%m%d".to_string(), // This should be ignored due to flexible parsing
                output_format: crate::OutputFormat::Human,
            };

            let (ok, holiday) = get_holiday(&opt)?;
            assert!(ok, "Failed to parse date: {}", date_str);
            assert_eq!(holiday, expected_holiday, "Wrong holiday for date: {}", date_str);
        }

        Ok(())
    }

    #[test]
    fn test_get_holidays_in_range() -> Result<(), Box<dyn std::error::Error>> {
        let holidays = get_holidays_in_range("2023-01-01", "2023-01-03")?;
        
        // Should find 元日 (New Year's Day) on 2023-01-01
        assert!(!holidays.is_empty());
        assert_eq!(holidays[0].0, "2023-01-01");
        assert_eq!(holidays[0].1, "元日");

        Ok(())
    }

    #[test]
    fn test_get_holidays_in_range_empty() -> Result<(), Box<dyn std::error::Error>> {
        let holidays = get_holidays_in_range("2023-02-02", "2023-02-05")?;
        
        // Should find no holidays in this range
        assert!(holidays.is_empty());

        Ok(())
    }

    #[test]
    fn test_get_holidays_in_range_invalid_dates() -> Result<(), Box<dyn std::error::Error>> {
        // Test with invalid date format
        let result = get_holidays_in_range("invalid-date", "2023-01-01");
        assert!(result.is_err());

        // Test with start date after end date
        let result = get_holidays_in_range("2023-12-31", "2023-01-01");
        assert!(result.is_err());

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

/// Get all holidays in a date range
pub fn get_holidays_in_range(start_date: &str, end_date: &str) -> Result<Vec<(String, &'static str)>> {
    let start = parse_date_flexible(start_date)
        .context("Failed to parse start date")?;
    let end = parse_date_flexible(end_date)
        .context("Failed to parse end date")?;
    
    if start > end {
        return Err(anyhow!("Start date must be before or equal to end date"));
    }
    
    let holidays = dates::dates();
    let mut result = Vec::new();
    
    let mut current = start;
    while current <= end {
        let date_str = current.format("%Y-%m-%d").to_string();
        if let Some(holiday_name) = holidays.get(date_str.as_str()) {
            result.push((date_str, *holiday_name));
        }
        current = current.succ_opt()
            .ok_or_else(|| anyhow!("Date overflow occurred"))?;
    }
    
    Ok(result)
}
