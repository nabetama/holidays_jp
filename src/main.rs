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

use anyhow::Result;
use std::{io::Write, process, str};

use clap::{arg, command, value_parser};
use holiday::holiday::get_holiday;
use chrono::Local;

use crate::holiday::generator::generate;

const CSV_FILE_URL: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
const OUT_FILE: &str = "./src/holiday/dates.rs";


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
}

impl CliOption {
    /// wrapped `println!` macro
    fn write(&self, write: &mut impl Write, name: &str) -> Result<()> {
        writeln!(write, "{} is holiday({})", self.date, name)?;
        Ok(())
    }
}

fn main() -> Result<()> {
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
        .get_matches();

    let date = matches.get_one::<String>("date")
        .map(|s| s.to_string())
        .unwrap_or_else(|| Local::now().format("%Y%m%d").to_string());
    let gen = matches.get_one::<bool>("gen").is_some();
    let date_format = matches.get_one::<String>("dateformat").unwrap().to_string();

    let opt = CliOption {
        date,
        gen,
        date_format,
    };

    if opt.gen {
        generate(CSV_FILE_URL, OUT_FILE)?;
        println!("generate process is done");
        process::exit(0x0100);
    }

    let (is_holiday, name) = get_holiday(&opt)?;

    if is_holiday {
        opt.write(&mut std::io::stdout(), name)?;
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
            gen: false,
            date_format: "%Y%m%d".to_string(),
        };

        let mut output: Vec<u8> = Vec::new();

        opt.write(&mut output, "Super Holiday!")?;
        assert_eq!(
            str::from_utf8(&output)?,
            "20230101 is holiday(Super Holiday!)\n"
        );

        Ok(())
    }
}
