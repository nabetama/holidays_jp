use crate::constants::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub holiday_data: HolidayDataConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayDataConfig {
    pub source_url: String,
    pub cache_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub strategy: CacheStrategy,
    pub max_age_hours: u64,
    pub etag_check_interval_hours: u64,
    pub force_refresh_on_startup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheStrategy {
    TimeBased,
    EtagBased,
    Hybrid,
    AlwaysRefresh,
    NeverRefresh,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            holiday_data: HolidayDataConfig {
                source_url: DEFAULT_SOURCE_URL.to_string(),
                cache_file: DEFAULT_CACHE_FILE.to_string(),
            },
            cache: CacheConfig {
                strategy: CacheStrategy::Hybrid,
                max_age_hours: 168, // 7 days - aligns with weekly GitHub Actions updates
                etag_check_interval_hours: 24, // Daily ETag check for emergency updates
                force_refresh_on_startup: false,
            },
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // 設定ファイルがあれば読み込み、なければデフォルト設定ファイルを生成
        if std::path::Path::new(CONFIG_FILE_NAME).exists() {
            println!("📄 Loading configuration from config.toml");
            let content = std::fs::read_to_string(CONFIG_FILE_NAME)?;
            let config: Config = toml::from_str(&content)?;
            println!("   Source URL: {}", config.holiday_data.source_url);
            println!("   Cache file: {}", config.holiday_data.cache_file);
            println!("   Cache strategy: {:?}", config.cache.strategy);
            Ok(config)
        } else {
            println!("📄 Creating default config.toml file");
            Self::create_default_config_file()?;
            Ok(Config::default())
        }
    }

    fn create_default_config_file() -> anyhow::Result<()> {
        let default_config = Config::default();
        let toml_content = toml::to_string_pretty(&default_config)?;
        std::fs::write(CONFIG_FILE_NAME, toml_content)?;
        println!("✅ Created default config.toml file");
        Ok(())
    }
}
