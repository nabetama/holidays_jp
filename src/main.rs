//! # holidays_jp
//!
//! A Cli tool for determines Japanese holidays.
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

pub mod holiday;

use anyhow::{Result, Context};
use std::{io::Write, process, str};

use clap::{arg, command, value_parser, ValueEnum, Subcommand};
use holiday::holiday::{get_holiday, get_holidays_in_range};
use chrono::Local;
use serde_json;

use crate::holiday::generator::generate;

const CSV_FILE_URL: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
const OUT_FILE: &str = "./src/holiday/dates.rs";

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

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    /// Check if a specific date is a holiday (default)
    Check {
        /// Date to check (default: today)
        #[arg(short, long)]
        date: Option<String>,
        
        /// Date format
        #[arg(short, long, default_value = "%Y%m%d")]
        dateformat: String,
        
        /// Output format
        #[arg(short, long, default_value = "human")]
        output: OutputFormat,
    },
    /// Update holiday data from official source
    Update,
    /// List holidays in a date range (future feature)
    List {
        /// Start date
        #[arg(short, long)]
        start: Option<String>,
        
        /// End date
        #[arg(short, long)]
        end: Option<String>,
        
        /// Output format
        #[arg(short, long, default_value = "human")]
        output: OutputFormat,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HolidayResult {
    date: String,
    is_holiday: bool,
    holiday_name: Option<String>,
}


/// A struct with command line arguments for CLI
///
/// # Example
///
/// ```no_run
/// let opt = CliOption {
///     date: "2023/01/01".to_string(),
///     gen: true,
///     date_format: "%Y/%m/%d".to_string()
/// };
/// ```
#[derive(Debug)]
pub struct CliOption {
    date: String,
    date_format: String,
    output_format: OutputFormat,
}

impl CliOption {
    /// Write holiday result based on output format
    fn write_result(&self, write: &mut impl Write, is_holiday: bool, holiday_name: Option<&str>) -> Result<()> {
        match self.output_format {
            OutputFormat::Human => {
                if is_holiday {
                    writeln!(write, "{} is holiday({})", self.date, holiday_name.unwrap_or(""))?;
                } else {
                    writeln!(write, "{} is not a holiday", self.date)?;
                }
            }
            OutputFormat::Json => {
                let result = HolidayResult {
                    date: self.date.clone(),
                    is_holiday,
                    holiday_name: holiday_name.map(|s| s.to_string()),
                };
                writeln!(write, "{}", serde_json::to_string(&result)?)?;
            }
            OutputFormat::Quiet => {
                if is_holiday {
                    writeln!(write, "{}", holiday_name.unwrap_or(""))?;
                }
                // For quiet mode, don't output anything for non-holidays
            }
        }
        Ok(())
    }
}

fn main() {
    if let Err(error) = run() {
        print_error_with_help(&error);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = command!("holidays_jp")
        .version("1.0")
        .author("Mao Nabeta")
        .about("A CLI tool for determining Japanese national holidays")
        .long_about("holidays_jp is a command-line tool that helps you check if specific dates are Japanese national holidays. It supports multiple date formats, various output formats, and can list holidays within a date range. The holiday data is based on the official CSV file provided by the Cabinet Office of Japan.")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            command!("check")
                .about("Check if a specific date is a holiday (default)")
                .long_about("Check whether a specific date is a Japanese national holiday. If no date is specified, today's date will be checked. Supports multiple date formats and output formats.")
                .arg(
                    arg!(--date <DATE>)
                        .help("Date to check (default: today)")
                        .long_help("The date to check for holidays. Supports various formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, YYYY年MM月DD日, etc.")
                        .short('d'),
                )
                .arg(
                    arg!(--dateformat <DATE_FORMAT>)
                        .help("Date format for parsing the input date")
                        .long_help("Specify the format of the input date. This is used as a fallback when automatic format detection fails. Default: %Y%m%d")
                        .default_value("%Y%m%d")
                        .short('f'),
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

    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            let date = sub_matches.get_one::<String>("date")
                .map(|s| s.to_string())
                .unwrap_or_else(|| Local::now().format("%Y%m%d").to_string());
            let date_format = sub_matches.get_one::<String>("dateformat").unwrap().to_string();
            let output_format = sub_matches.get_one::<OutputFormat>("output").unwrap().clone();

            let opt = CliOption {
                date,
                date_format,
                output_format,
            };

            let (is_holiday, name) = get_holiday(&opt)
                .context("Failed to check holiday status. Please verify your date format.")?;

            opt.write_result(&mut std::io::stdout(), is_holiday, if name.is_empty() { None } else { Some(name) })
                .context("Failed to write output. Please check your terminal settings.")?;
        }
        Some(("update", _)) => {
            println!("🔄 Updating holiday data from official source...");
            generate(CSV_FILE_URL, OUT_FILE)
                .context("Failed to update holiday data. Please check your internet connection and try again.")?;
            println!("✅ Holiday data updated successfully!");
        }
        Some(("list", sub_matches)) => {
            let start = sub_matches.get_one::<String>("start");
            let end = sub_matches.get_one::<String>("end");
            let output_format = sub_matches.get_one::<OutputFormat>("output").unwrap().clone();
            
            if start.is_none() || end.is_none() {
                eprintln!("❌ Error: Both --start and --end dates are required for list command");
                eprintln!("💡 Example: ./holidays_jp list --start 2023-01-01 --end 2023-12-31");
                return Ok(());
            }
            
            let start_date = start.unwrap();
            let end_date = end.unwrap();
            
            let holidays = get_holidays_in_range(start_date, end_date)
                .context("Failed to get holidays in range. Please check your date formats.")?;
            
            if holidays.is_empty() {
                match output_format {
                    OutputFormat::Human => {
                        println!("No holidays found in the specified range ({} to {})", start_date, end_date);
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
                        let holiday_list: Vec<HolidayResult> = holidays.into_iter()
                            .map(|(date, name)| HolidayResult {
                                date,
                                is_holiday: true,
                                holiday_name: Some(name.to_string()),
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
        }
        None => {
            // Default behavior: check today's date
            let opt = CliOption {
                date: Local::now().format("%Y%m%d").to_string(),
                date_format: "%Y%m%d".to_string(),
                output_format: OutputFormat::Human,
            };

            let (is_holiday, name) = get_holiday(&opt)
                .context("Failed to check holiday status. Please verify your date format.")?;

            opt.write_result(&mut std::io::stdout(), is_holiday, if name.is_empty() { None } else { Some(name) })
                .context("Failed to write output. Please check your terminal settings.")?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_output_result() -> Result<()> {
        let opt = CliOption {
            date: "20230101".to_string(),
            date_format: "%Y%m%d".to_string(),
            output_format: OutputFormat::Human,
        };

        let mut output: Vec<u8> = Vec::new();

        opt.write_result(&mut output, true, Some("Super Holiday!"))?;
        assert_eq!(
            str::from_utf8(&output)?,
            "20230101 is holiday(Super Holiday!)\n"
        );

        Ok(())
    }

    #[test]
    fn test_json_output() -> Result<()> {
        let opt = CliOption {
            date: "20230101".to_string(),
            date_format: "%Y%m%d".to_string(),
            output_format: OutputFormat::Json,
        };

        let mut output: Vec<u8> = Vec::new();

        opt.write_result(&mut output, true, Some("元日"))?;
        let json_str = str::from_utf8(&output)?;
        let result: HolidayResult = serde_json::from_str(json_str.trim())?;
        
        assert_eq!(result.date, "20230101");
        assert!(result.is_holiday);
        assert_eq!(result.holiday_name, Some("元日".to_string()));

        Ok(())
    }
}
