//! # holidays_jp
//!
//! A CLI tool for determining Japanese national holidays.
//! #holiday #Japan #Japanese
//!
//! 日本の祝日判定を行うCLIツール
//!
//! ## Usage
//! ```
//! $ cargo build --release
//! $ ./target/release/holidays_jp -h
//! holidays_jp is determines holiday in Japan
//!
//! Usage: holidays_jp [OPTIONS]
//!
//! Options:
//!   -d, --date <DATE>               a date string, such as 20230211 (%Y%m%d) [default: today]
//!   -g, --gen <BOOL>                generates a new Japanese national holidays data [possible values: true, false]
//!   -f, --dateformat <DATE_FORMAT>  Specify the date format to pass as a command line argument [default: %Y%m%d]
//!   -h, --help                      Print help
//!   -V, --version                   Print version
//! ```

pub mod cache;
pub mod config;
pub mod constants;
pub mod holiday_service;

use anyhow::{Context, Result};
use std::{io::Write, process, str};

use clap::{arg, command, value_parser, ValueEnum};
use holiday_service::HolidayService;
use serde_json;

/// Print user-friendly error message with usage examples
fn print_error_with_help(error: &anyhow::Error) {
    eprintln!("❌ Error: {}", error);

    // Add specific help based on error type
    if error.downcast_ref::<chrono::format::ParseError>().is_some() {
        eprintln!("\n💡 Date parsing help:");
        eprintln!("   Supported formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日");
        eprintln!("   Examples:");
        eprintln!("     ./holidays_jp -d 2023-01-01");
        eprintln!("     ./holidays_jp -d 2023/01/01");
        eprintln!("     ./holidays_jp -d 2023年1月1日");
        eprintln!("     ./holidays_jp -d 20230101");
    } else if error.downcast_ref::<serde_json::Error>().is_some() {
        eprintln!("\n💡 JSON output error:");
        eprintln!("   This might be due to invalid JSON serialization.");
        eprintln!("   Try using a different output format: -o human");
    } else if error.downcast_ref::<std::io::Error>().is_some() {
        eprintln!("\n💡 I/O error:");
        eprintln!("   Check if you have write permissions and sufficient disk space.");
    }

    eprintln!("\n📖 For more help, run: ./holidays_jp --help");
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    /// Human-readable format (default)
    Human,
    /// JSON format
    Json,
    /// Quiet format (only show holiday name, nothing for non-holidays)
    Quiet,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HolidayResult {
    date: String,
    is_holiday: bool,
    holiday_name: Option<String>,
}

fn main() {
    if let Err(error) = run() {
        print_error_with_help(&error);
        process::exit(1);
    }
}

#[tokio::main]
async fn run() -> Result<()> {
    let config = config::Config::load()?;

    let matches = command!(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A CLI tool for determining Japanese national holidays")
        .long_about("holidays_jp is a command-line tool that helps you check if specific dates are Japanese national holidays. It supports multiple date formats, various output formats, and can list holidays within a date range. The holiday data is based on the official CSV file provided by the Cabinet Office of Japan.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            command!("check")
                .about("Check if a specific date is a holiday (default)")
                .long_about("Check whether a specific date is a Japanese national holiday. If no date is specified, today's date will be checked. Supports multiple date formats and output formats.")
                .arg(
                    arg!([DATE])
                        .help("Date to check (default: today)")
                        .long_help("The date to check for holidays. Supports various formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.")
                )
                .arg(
                    arg!(--date <DATE>)
                        .help("Date to check (default: today)")
                        .long_help("The date to check for holidays. Supports various formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.")
                        .short('d')
                        .conflicts_with("DATE"),
                )
                .arg(
                    arg!(--output <OUTPUT_FORMAT>)
                        .help("Output format")
                        .long_help("Choose how to display the result: human (readable), json (structured), or quiet (minimal)")
                        .value_parser(value_parser!(OutputFormat))
                        .default_value("human")
                        .short('o'),
                ),
        )
        .subcommand(
            command!("update")
                .about("Update holiday data from official source")
                .long_about("Download the latest Japanese national holiday data from the official Cabinet Office CSV file and update the local database. This command requires an internet connection."),
        )
        .subcommand(
            command!("list")
                .about("List holidays in a date range")
                .long_about("List all Japanese national holidays within a specified date range. Both start and end dates are required. Supports multiple date formats and output formats.")
                .arg(
                    arg!(--start <START_DATE>)
                        .help("Start date of the range")
                        .long_help("The start date of the range to search for holidays. Supports various formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.")
                        .short('s'),
                )
                .arg(
                    arg!(--end <END_DATE>)
                        .help("End date of the range")
                        .long_help("The end date of the range to search for holidays. Supports various formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.")
                        .short('e'),
                )
                .arg(
                    arg!(--output <OUTPUT_FORMAT>)
                        .help("Output format")
                        .long_help("Choose how to display the results: human (readable list), json (structured data), or quiet (minimal format)")
                        .value_parser(value_parser!(OutputFormat))
                        .default_value("human")
                        .short('o'),
                ),
        )
        .get_matches();

    // 祝日サービスを初期化
    let mut holiday_service = HolidayService::new(config.clone());
    holiday_service.initialize().await
        .context("Failed to initialize holiday service. Please check your internet connection and try again.")?;

    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            // Check positional argument first, then fall back to --date option
            let date = sub_matches
                .get_one::<String>("DATE")
                .or_else(|| sub_matches.get_one::<String>("date"))
                .map(|s| s.to_string())
                .unwrap_or_else(|| HolidayService::get_today_date());
            let output_format = sub_matches
                .get_one::<OutputFormat>("output")
                .unwrap()
                .clone();

            let (is_holiday, holiday_name) = holiday_service
                .get_holiday(&date)
                .context("Failed to check holiday status. Please verify your date format.")?;

            write_holiday_result(&date, is_holiday, holiday_name.as_deref(), output_format)?;
        }
        Some(("update", _)) => {
            println!("🔄 Updating holiday data from official source...");
            // 強制更新のためにキャッシュを削除
            let cache_path = &config.holiday_data.cache_file;
            if std::path::Path::new(cache_path).exists() {
                std::fs::remove_file(cache_path)?;
            }
            // 再初期化してデータをダウンロード
            holiday_service.initialize().await
                .context("Failed to update holiday data. Please check your internet connection and try again.")?;
            println!("✅ Holiday data updated successfully!");
        }
        Some(("list", sub_matches)) => {
            let start = sub_matches.get_one::<String>("start");
            let end = sub_matches.get_one::<String>("end");
            let output_format = sub_matches
                .get_one::<OutputFormat>("output")
                .unwrap()
                .clone();

            if start.is_none() || end.is_none() {
                eprintln!("❌ Error: Both --start and --end dates are required for list command");
                eprintln!("💡 Example: ./holidays_jp list --start 2023-01-01 --end 2023-12-31");
                return Ok(());
            }

            let start_date = start.unwrap();
            let end_date = end.unwrap();

            let holidays = holiday_service
                .get_holidays_in_range(start_date, end_date)
                .context("Failed to get holidays in range. Please check your date formats.")?;

            write_holidays_list(start_date, end_date, &holidays, output_format)?;
        }
        None => {
            // Default behavior: check today's date
            let today = HolidayService::get_today_date();
            let (is_holiday, holiday_name) = holiday_service
                .get_holiday(&today)
                .context("Failed to check holiday status. Please verify your date format.")?;

            write_holiday_result(
                &today,
                is_holiday,
                holiday_name.as_deref(),
                OutputFormat::Human,
            )?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn write_holiday_result(
    date: &str,
    is_holiday: bool,
    holiday_name: Option<&str>,
    output_format: OutputFormat,
) -> Result<()> {
    match output_format {
        OutputFormat::Human => {
            if is_holiday {
                writeln!(
                    std::io::stdout(),
                    "{} is holiday({})",
                    date,
                    holiday_name.unwrap_or("")
                )?;
            } else {
                writeln!(std::io::stdout(), "{} is not a holiday", date)?;
            }
        }
        OutputFormat::Json => {
            let result = HolidayResult {
                date: date.to_string(),
                is_holiday,
                holiday_name: holiday_name.map(|s| s.to_string()),
            };
            writeln!(std::io::stdout(), "{}", serde_json::to_string(&result)?)?;
        }
        OutputFormat::Quiet => {
            if is_holiday {
                writeln!(std::io::stdout(), "{}", holiday_name.unwrap_or(""))?;
            }
            // For quiet mode, don't output anything for non-holidays
        }
    }
    Ok(())
}

fn write_holidays_list(
    start_date: &str,
    end_date: &str,
    holidays: &[(String, String)],
    output_format: OutputFormat,
) -> Result<()> {
    if holidays.is_empty() {
        match output_format {
            OutputFormat::Human => {
                println!(
                    "No holidays found in the specified range ({} to {})",
                    start_date, end_date
                );
            }
            OutputFormat::Json => {
                let result = serde_json::json!({
                    "start_date": start_date,
                    "end_date": end_date,
                    "holidays": []
                });
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            OutputFormat::Quiet => {
                // No output for quiet mode when no holidays found
            }
        }
    } else {
        match output_format {
            OutputFormat::Human => {
                println!("Holidays in range ({} to {}):", start_date, end_date);
                for (date, name) in holidays {
                    println!("  {} - {}", date, name);
                }
            }
            OutputFormat::Json => {
                let holiday_list: Vec<HolidayResult> = holidays
                    .iter()
                    .map(|(date, name)| HolidayResult {
                        date: date.clone(),
                        is_holiday: true,
                        holiday_name: Some(name.clone()),
                    })
                    .collect();
                let result = serde_json::json!({
                    "start_date": start_date,
                    "end_date": end_date,
                    "holidays": holiday_list
                });
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            OutputFormat::Quiet => {
                for (date, name) in holidays {
                    println!("{} - {}", date, name);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_holiday_result_human() -> Result<()> {
        // テストは実際の出力を確認するため、stdoutをキャプチャする必要がある
        // ここでは基本的な動作確認のみ
        write_holiday_result("20230101", true, Some("元日"), OutputFormat::Human)?;
        write_holiday_result("20230102", false, None, OutputFormat::Human)?;
        Ok(())
    }

    #[test]
    fn test_write_holiday_result_json() -> Result<()> {
        // テストは実際の出力を確認するため、stdoutをキャプチャする必要がある
        // ここでは基本的な動作確認のみ
        write_holiday_result("20230101", true, Some("元日"), OutputFormat::Json)?;
        Ok(())
    }
}
