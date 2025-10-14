//! Minimal constants that are truly constant and not configuration values
//! 
//! These are values that should never change and don't belong in config files.
//! All user-configurable values are defined in config.toml.

/// Application metadata (not user-configurable)
pub const APP_NAME: &str = "holidays_jp";
pub const APP_VERSION: &str = "1.0";
pub const APP_AUTHOR: &str = "Mao Nabeta";

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Supported date formats for flexible parsing (technical implementation detail)
pub const SUPPORTED_DATE_FORMATS: &[&str] = &[
    "%Y%m%d",        // 20230101
    "%Y-%m-%d",      // 2023-01-01
    "%Y/%m/%d",      // 2023/01/01
    "%Y年%m月%d日",   // 2023年1月1日
    "%m/%d/%Y",      // 01/01/2023
    "%d/%m/%Y",      // 01/01/2023 (European format)
    "%Y.%m.%d",      // 2023.01.01
];

/// Valid cache strategy options (for validation)
pub const CACHE_STRATEGY_OPTIONS: &[&str] = &[
    "TimeBased",
    "EtagBased", 
    "Hybrid",
    "AlwaysRefresh",
    "NeverRefresh",
];

/// Default configuration values (used only when creating initial config.toml)
pub const DEFAULT_SOURCE_URL: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
pub const DEFAULT_CACHE_FILE: &str = "./data/holidays.json";
