mod holiday;

use anyhow::Result;
use std::process;

use clap::{arg, command, value_parser};
use holiday::{
    dates,
    holiday::{find_holiday, get_date},
};

use crate::holiday::generator::generate;

const CSV_FILE_URL: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
const OUT_FILE: &str = "./src/holiday/dates.rs";

#[derive(Debug)]
pub struct CliOption {
    date: String,
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

    match matches.get_one::<bool>("gen") {
        Some(_) => {
            generate(CSV_FILE_URL, OUT_FILE)?;
            println!("generate process is done");
            process::exit(0x0100);
        }
        None => {}
    }

    let date = get_date(matches.get_one::<String>("date").unwrap())?;
    let opt = CliOption { date };
    let holidays = dates::dates();

    match find_holiday(holidays, opt, &mut std::io::stdout()) {
        Ok(_) => process::exit(0x0100),
        Err(err) => {
            eprintln!("{:?}", err.to_string())
        }
    }

    Ok(())
}
