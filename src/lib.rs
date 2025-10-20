//! # holidays_jp
//!
//! A Rust library and CLI tool for determining Japanese national holidays.
//!
//! This library provides functionality to check if a specific date is a Japanese national holiday
//! and to list holidays within a date range. The holiday data is based on the official CSV file
//! provided by the Cabinet Office of Japan.
//!
//! ## Features
//!
//! - Check if a specific date is a Japanese national holiday
//! - List all holidays within a date range
//! - Support for multiple date formats (YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.)
//! - Automatic caching of holiday data with configurable update strategies
//! - Async/await support using tokio
//!
//! ## Library Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! holidays_jp = "0.2"
//! tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
//! ```
//!
//! ## Examples
//!
//! ### Check if a specific date is a holiday
//!
//! ```rust,no_run
//! use holidays_jp::{HolidayService, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let mut service = HolidayService::new(config);
//!     service.initialize().await?;
//!
//!     let (is_holiday, holiday_name) = service.get_holiday("2023-01-01")?;
//!     if is_holiday {
//!         println!("2023-01-01 is a holiday: {}", holiday_name.unwrap());
//!     } else {
//!         println!("2023-01-01 is not a holiday");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### List holidays in a date range
//!
//! ```rust,no_run
//! use holidays_jp::{HolidayService, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let mut service = HolidayService::new(config);
//!     service.initialize().await?;
//!
//!     let holidays = service.get_holidays_in_range("2023-01-01", "2023-12-31")?;
//!     for (date, name) in holidays {
//!         println!("{}: {}", date, name);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod cache;
pub mod config;
pub mod constants;
pub mod holiday_service;

// Re-export main types for easier use
pub use config::Config;
pub use holiday_service::HolidayService;
