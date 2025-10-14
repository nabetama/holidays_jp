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
    pub source_url: String,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub version: String,
    pub cache_duration_hours: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub metadata: CacheMetadata,
    pub holidays: HashMap<String, String>,
}

pub struct HolidayCache {
    config: Config,
    cache_path: PathBuf,
}

impl HolidayCache {
    pub fn new(config: Config) -> Self {
        let cache_path = PathBuf::from(&config.holiday_data.cache_file);
        Self { config, cache_path }
    }

    pub async fn get_holidays(&self) -> Result<HashMap<String, String>> {
        if self.config.cache.force_refresh_on_startup {
            println!("🔄 Force refresh enabled, downloading fresh data...");
            return self.download_and_cache().await;
        }

        if !self.cache_path.exists() {
            println!("📥 No cache file found, downloading data...");
            return self.download_and_cache().await;
        }

        let cache_data = self.load_cache_data()?;
        
        if self.should_refresh_cache(&cache_data.metadata).await? {
            println!("🔄 Cache is stale, downloading fresh data...");
            return self.download_and_cache().await;
        }

        // 5. キャッシュから読み込み
        println!("✅ Using cached holiday data");
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
        println!("🔍 Checking cache freshness...");
        println!("   Last updated: {}", metadata.last_updated);
        println!("   Cache age: {} hours", self.get_cache_age_hours(metadata));
        println!("   Strategy: {:?}", self.config.cache.strategy);

        match &self.config.cache.strategy {
            CacheStrategy::AlwaysRefresh => {
                println!("   ⚡ Force refresh enabled");
                Ok(true)
            }
            
            CacheStrategy::NeverRefresh => {
                println!("   🔒 Cache refresh disabled");
                Ok(false)
            }
            
            CacheStrategy::TimeBased => {
                self.should_refresh_time_based(metadata)
            }
            
            CacheStrategy::EtagBased => {
                self.should_refresh_etag_based(metadata).await
            }
            
            CacheStrategy::Hybrid => {
                self.should_refresh_hybrid(metadata).await
            }
        }
    }

    fn should_refresh_time_based(&self, metadata: &CacheMetadata) -> Result<bool> {
        let cache_age_hours = self.get_cache_age_hours(metadata);
        let max_age_hours = self.config.cache.max_age_hours;
        
        let should_refresh = cache_age_hours > max_age_hours;
        
        if should_refresh {
            println!("   ⏰ Cache expired ({}h > {}h)", cache_age_hours, max_age_hours);
        } else {
            println!("   ✅ Cache is fresh ({}h <= {}h)", cache_age_hours, max_age_hours);
        }
        
        Ok(should_refresh)
    }

    async fn should_refresh_etag_based(&self, metadata: &CacheMetadata) -> Result<bool> {
        if metadata.etag.is_none() {
            println!("   ⚠️  No ETag available, falling back to time-based check");
            return self.should_refresh_time_based(metadata);
        }

        match self.check_remote_etag().await {
            Ok(Some(remote_etag)) => {
                let should_refresh = metadata.etag.as_ref() != Some(&remote_etag);
                
                if should_refresh {
                    println!("   🔄 ETag changed: {} -> {}", 
                        metadata.etag.as_deref().unwrap_or("None"), 
                        remote_etag
                    );
                } else {
                    println!("   ✅ ETag unchanged: {}", remote_etag);
                }
                
                Ok(should_refresh)
            }
            Ok(None) => {
                println!("   ⚠️  No ETag from server, using time-based check");
                self.should_refresh_time_based(metadata)
            }
            Err(e) => {
                println!("   ❌ Failed to check ETag: {}, using time-based check", e);
                self.should_refresh_time_based(metadata)
            }
        }
    }

    async fn should_refresh_hybrid(&self, metadata: &CacheMetadata) -> Result<bool> {
        let cache_age_hours = self.get_cache_age_hours(metadata);
        
        if cache_age_hours > self.config.cache.max_age_hours {
            println!("   ⏰ Cache too old ({}h > {}h), forcing refresh", 
                cache_age_hours, self.config.cache.max_age_hours);
            return Ok(true);
        }
        
        if cache_age_hours > self.config.cache.etag_check_interval_hours {
            println!("   🔍 ETag check interval reached ({}h > {}h)", 
                cache_age_hours, self.config.cache.etag_check_interval_hours);
            return self.should_refresh_etag_based(metadata).await;
        }
        
        println!("   ✅ Cache is fresh, no refresh needed");
        Ok(false)
    }

    fn get_cache_age_hours(&self, metadata: &CacheMetadata) -> u64 {
        let now = Utc::now();
        let duration = now - metadata.last_updated;
        duration.num_hours() as u64
    }

    async fn check_remote_etag(&self) -> Result<Option<String>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        
        let response = client
            .head(&self.config.holiday_data.source_url)
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
        println!("📥 Downloading holiday data from: {}", self.config.holiday_data.source_url);
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        let response = client
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

        let last_modified = response.headers()
            .get("last-modified")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let body = response.text_with_charset("shift-jis").await?;
        let holidays = self.parse_csv(&body)?;

        // キャッシュディレクトリを作成
        if let Some(parent) = self.cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // キャッシュに保存
        let cache_data = CacheData {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                source_url: self.config.holiday_data.source_url.clone(),
                etag,
                last_modified,
                version: "1.0".to_string(),
                cache_duration_hours: self.config.cache.max_age_hours,
            },
            holidays: holidays.clone(),
        };

        let json = serde_json::to_string_pretty(&cache_data)?;
        std::fs::write(&self.cache_path, json)?;

        println!("✅ Holiday data cached successfully");
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
