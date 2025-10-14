use crate::cache::HolidayCache;
use crate::config::Config;
use crate::constants::*;
use anyhow::Result;
use chrono::{Local, NaiveDate};
use std::collections::HashMap;

pub struct HolidayService {
    cache: HolidayCache,
    holidays: Option<HashMap<String, String>>,
}

impl HolidayService {
    pub fn new(config: Config) -> Self {
        Self {
            cache: HolidayCache::new(config),
            holidays: None,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        self.holidays = Some(self.cache.get_holidays().await?);
        Ok(())
    }

    pub fn get_holiday(&self, date: &str) -> Result<(bool, Option<String>)> {
        let holidays = self.holidays.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Holiday service not initialized"))?;

        let parsed_date = self.parse_date_flexible(date)?;
        let formatted_date = parsed_date.format("%Y-%m-%d").to_string();

        if let Some(holiday_name) = holidays.get(&formatted_date) {
            Ok((true, Some(holiday_name.clone())))
        } else {
            Ok((false, None))
        }
    }

    pub fn get_holidays_in_range(&self, start_date: &str, end_date: &str) -> Result<Vec<(String, String)>> {
        let holidays = self.holidays.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Holiday service not initialized"))?;

        let start = self.parse_date_flexible(start_date)?;
        let end = self.parse_date_flexible(end_date)?;
        
        if start > end {
            return Err(anyhow::anyhow!("Start date must be before or equal to end date"));
        }
        
        let mut result = Vec::new();
        let mut current = start;
        
        while current <= end {
            let date_str = current.format("%Y-%m-%d").to_string();
            if let Some(holiday_name) = holidays.get(&date_str) {
                result.push((date_str, holiday_name.clone()));
            }
            current = current.succ_opt()
                .ok_or_else(|| anyhow::anyhow!("Date overflow occurred"))?;
        }
        
        Ok(result)
    }

    fn parse_date_flexible(&self, date_str: &str) -> Result<NaiveDate> {
        for format in SUPPORTED_DATE_FORMATS {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }
        
        Err(anyhow::anyhow!(
            "Invalid date format: '{}'. Please use one of these formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, MM/DD/YYYY, DD/MM/YYYY, or YYYY.MM.DD", 
            date_str
        ))
    }

    pub fn get_today_date() -> String {
        Local::now().format("%Y%m%d").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_holiday_service_initialization() {
        let config = Config::default();
        let service = HolidayService::new(config);

        assert!(service.holidays.is_none());
    }

    #[test]
    fn test_parse_date_flexible() {
        let config = Config::default();
        let service = HolidayService::new(config);
        
        let test_cases = vec![
            ("2023-01-01", "2023-01-01"),
            ("2023/01/01", "2023-01-01"),
            ("2023年1月1日", "2023-01-01"),
            ("20230101", "2023-01-01"),
        ];

        for (input, expected) in test_cases {
            let result = service.parse_date_flexible(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            assert_eq!(result.unwrap().format("%Y-%m-%d").to_string(), expected);
        }
    }
}
