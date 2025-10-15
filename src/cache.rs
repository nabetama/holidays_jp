use crate::config::{Config, CacheStrategy};
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub last_updated: DateTime<Utc>,
    pub etag: Option<String>,
    pub last_etag_check: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub metadata: CacheMetadata,
    pub holidays: HashMap<String, String>,
}

pub struct HolidayCache {
    config: Config,
    cache_path: PathBuf,
    http_client: reqwest::Client,
}

impl HolidayCache {
    pub fn new(config: Config) -> Self {
        let cache_path = PathBuf::from(&config.holiday_data.cache_file);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        Self { config, cache_path, http_client }
    }

    pub async fn get_holidays(&self) -> Result<HashMap<String, String>> {
        if self.config.cache.force_refresh_on_startup {
            return self.download_and_cache().await;
        }

        if !self.cache_path.exists() {
            return self.download_and_cache().await;
        }

        let cache_data = self.load_cache_data()?;

        if self.should_refresh_cache(&cache_data.metadata).await? {
            return self.download_and_cache().await;
        }

        Ok(cache_data.holidays)
    }

    fn load_cache_data(&self) -> Result<CacheData> {
        let content = std::fs::read_to_string(&self.cache_path)
            .context("Failed to read cache file")?;
        
        let cache_data: CacheData = serde_json::from_str(&content)
            .context("Failed to parse cache file")?;
        
        Ok(cache_data)
    }

    async fn should_refresh_cache(&self, metadata: &CacheMetadata) -> Result<bool> {
        match &self.config.cache.strategy {
            CacheStrategy::AlwaysRefresh => Ok(true),
            CacheStrategy::NeverRefresh => Ok(false),
            CacheStrategy::TimeBased => self.should_refresh_time_based(metadata),
            CacheStrategy::EtagBased => self.should_refresh_etag_based(metadata).await,
            CacheStrategy::Hybrid => self.should_refresh_hybrid(metadata).await,
        }
    }

    fn should_refresh_time_based(&self, metadata: &CacheMetadata) -> Result<bool> {
        let cache_age_hours = self.get_cache_age_hours(metadata);
        let max_age_hours = self.config.cache.max_age_hours;
        Ok(cache_age_hours > max_age_hours)
    }

    async fn should_refresh_etag_based(&self, metadata: &CacheMetadata) -> Result<bool> {
        if metadata.etag.is_none() {
            return self.should_refresh_time_based(metadata);
        }

        match self.check_remote_etag().await {
            Ok(Some(remote_etag)) => {
                Ok(metadata.etag.as_ref() != Some(&remote_etag))
            }
            Ok(None) | Err(_) => {
                self.should_refresh_time_based(metadata)
            }
        }
    }

    async fn should_refresh_hybrid(&self, metadata: &CacheMetadata) -> Result<bool> {
        let cache_age_hours = self.get_cache_age_hours(metadata);

        // Force refresh if cache is too old
        if cache_age_hours > self.config.cache.max_age_hours {
            return Ok(true);
        }

        // Check if we should perform an ETag check
        let should_check_etag = match metadata.last_etag_check {
            Some(last_check) => {
                let hours_since_check = (Utc::now() - last_check).num_hours() as u64;
                hours_since_check > self.config.cache.etag_check_interval_hours
            }
            None => true, // Never checked, do it now
        };

        if should_check_etag {
            // Perform ETag check and update timestamp
            return self.should_refresh_etag_based(metadata).await;
        }

        Ok(false)
    }

    fn get_cache_age_hours(&self, metadata: &CacheMetadata) -> u64 {
        let now = Utc::now();
        let duration = now - metadata.last_updated;
        duration.num_hours() as u64
    }

    async fn check_remote_etag(&self) -> Result<Option<String>> {
        let response = self.http_client
            .head(&self.config.holiday_data.source_url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        if response.status().is_success() {
            let etag = response.headers()
                .get("etag")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            Ok(etag)
        } else {
            Err(anyhow::anyhow!("HTTP error: {}", response.status()))
        }
    }

    async fn download_and_cache(&self) -> Result<HashMap<String, String>> {
        let response = self.http_client
            .get(&self.config.holiday_data.source_url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to download data: {}", response.status()));
        }

        let etag = response.headers()
            .get("etag")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let body = response.text_with_charset("shift-jis").await?;
        let holidays = self.parse_csv(&body)?;

        // Create cache directory if needed
        if let Some(parent) = self.cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Save to cache
        let now = Utc::now();
        let cache_data = CacheData {
            metadata: CacheMetadata {
                last_updated: now,
                etag,
                last_etag_check: Some(now),
            },
            holidays: holidays.clone(),
        };

        let json = serde_json::to_string_pretty(&cache_data)?;
        std::fs::write(&self.cache_path, json)?;

        Ok(holidays)
    }

    fn parse_csv(&self, csv_content: &str) -> Result<HashMap<String, String>> {
        let mut holidays = HashMap::new();
        let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());

        for result in rdr.records() {
            let record = result?;
            if record.len() >= 2 {
                let date_str = &record[0];
                let holiday_name = &record[1];
                
                // 日付を YYYY-MM-DD 形式に変換
                if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y/%m/%d") {
                    let formatted_date = date.format("%Y-%m-%d").to_string();
                    holidays.insert(formatted_date, holiday_name.to_string());
                }
            }
        }

        Ok(holidays)
    }
}
