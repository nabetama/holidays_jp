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

use clap::{arg, command, value_parser, ValueEnum};
use holiday::holiday::get_holiday;
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
    gen: bool,
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
        .about("holidays_jp is determines holiday in Japan")
        .arg(
            arg!(--date <DATE>)
                .required(false)
                .help("a date string, such as 20230211 (%Y%m%d) [default: today]")
                .short('d'),
        )
        .arg(
            arg!(--gen <GEN>)
                .required(false)
                .help("generates a new Japanese national holidays data")
                .value_name("BOOL")
                .value_parser(value_parser!(bool))
                .default_missing_value("false")
                .short('g'),
        )
        .arg(
            arg!(--dateformat <DATE_FORMAT>)
                .required(false)
                .help("Specify the date format to pass as a command line argument")
                .default_value("%Y%m%d")
                .short('f'),
        )
        .arg(
            arg!(--output <OUTPUT_FORMAT>)
                .required(false)
                .help("Output format")
                .value_parser(value_parser!(OutputFormat))
                .default_value("human")
                .short('o'),
        )
        .get_matches();

    let date = matches.get_one::<String>("date")
        .map(|s| s.to_string())
        .unwrap_or_else(|| Local::now().format("%Y%m%d").to_string());
    let gen = matches.get_one::<bool>("gen").is_some();
    let date_format = matches.get_one::<String>("dateformat").unwrap().to_string();
    let output_format = matches.get_one::<OutputFormat>("output").unwrap().clone();

    let opt = CliOption {
        date,
        gen,
        date_format,
        output_format,
    };

    if opt.gen {
        println!("🔄 Generating holiday data from official source...");
        generate(CSV_FILE_URL, OUT_FILE)
            .context("Failed to generate holiday data. Please check your internet connection and try again.")?;
        println!("✅ Holiday data generation completed successfully!");
        return Ok(());
    }

    let (is_holiday, name) = get_holiday(&opt)
        .context("Failed to check holiday status. Please verify your date format.")?;

    opt.write_result(&mut std::io::stdout(), is_holiday, if name.is_empty() { None } else { Some(name) })
        .context("Failed to write output. Please check your terminal settings.")?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_output_result() -> Result<()> {
        let opt = CliOption {
            date: "20230101".to_string(),
            gen: false,
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
            gen: false,
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
