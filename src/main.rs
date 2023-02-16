mod holiday;

use anyhow::Result;
use chrono::NaiveDate;
use std::{io::Write, process};

use clap::{arg, command, value_parser};
use holiday::holiday::{get_date, get_holiday};

use crate::holiday::generator::generate;

const CSV_FILE_URL: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
const OUT_FILE: &str = "./src/holiday/dates.rs";

/// A struct with command line arguments for CLI
///
/// # Example
///
/// ```no_run
/// let opt = CliOption { date: "2023/01/01", gen: true };
/// ```
#[derive(Debug)]
pub struct CliOption {
    date: NaiveDate,
    gen: bool,
}

impl CliOption {
    fn write(&self, write: &mut impl Write, name: &str) -> Result<()> {
        writeln!(write, "{} is holiday({})", self.date, name)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let matches = command!("holiday-jp")
        .version("1.0")
        .author("Mao Nabeta")
        .about("holiday-jp is determines holiday in Japan")
        .arg(
            arg!(--date <DATE>)
                .required(false)
                .default_value("")
                .help("a date string, such as 2023/02/11 (%Y/%m/%d)")
                .short('d'),
        )
        .arg(
            arg!(--gen <GEN>)
                .required(false)
                .help("generate new syukujitsu data")
                .value_name("BOOL")
                .value_parser(value_parser!(bool))
                .default_missing_value("false")
                .short('g'),
        )
        .get_matches();

    let date = get_date(matches.get_one::<String>("date").unwrap())?;
    let gen = matches.get_one::<bool>("gen").is_some();

    let opt = CliOption { date, gen };

    if opt.gen {
        generate(CSV_FILE_URL, OUT_FILE)?;
        println!("generate process is done");
        process::exit(0x0100);
    }

    let (is_holiday, name) = get_holiday(opt.date);

    if is_holiday {
        opt.write(&mut std::io::stdout(), name)?;
    }

    Ok(())
}
