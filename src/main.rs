mod holiday;
mod reader;

use anyhow::Result;
use std::process;

use clap::{arg, command};
use holiday::holiday::{find_holiday, get_date};

use crate::reader::csv_reader::get_holidays;

#[derive(Debug)]
pub struct CliOption {
    file: String,
    date: String,
}

fn main() -> Result<()> {
    let matches = command!("Holiday")
        .version("1.0")
        .author("Mao Nabeta")
        .about("Holiday is determines holiday in Japan")
        .arg(
            arg!(--file <FILE>)
                .required(false)
                .default_value("assets/syukujitsu.csv")
                .help("csv file with list of Japanese holidays")
                .short('f'),
        )
        .arg(
            arg!(--date <DATE>)
                .required(false)
                .default_value("")
                .help("a date string, such as 2023/02/11 (%Y/%m/%d)")
                .short('d'),
        )
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap().to_string();
    let date = get_date(matches.get_one::<String>("date").unwrap())?;

    let opt = CliOption { file, date };
    let holidays = get_holidays(&opt.file)?;

    let result = find_holiday(holidays, opt, &mut std::io::stdout());

    match result {
        Ok(_) => process::exit(0x0100),
        Err(err) => {
            eprintln!("{:?}", err.to_string())
        }
    }

    Ok(())
}
