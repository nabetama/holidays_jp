mod reader;

use std::error::Error;

use chrono::{Local, NaiveDate, ParseError};
use clap::{arg, command};

use crate::reader::csv_reader::get_holidays;

#[derive(Debug)]
struct CliOption {
    file: String,
    date: String,
}

#[test]
fn test_get_date() -> Result<(), ParseError> {
    let dt = get_date("2023/01/01")?;
    assert_eq!(dt, "2023/01/01");

    Ok(())
}

fn get_date(date_arg: &str) -> Result<String, ParseError> {
    if date_arg.to_string().len() > 0 {
        match NaiveDate::parse_from_str(date_arg, "%Y/%m/%d") {
            Ok(dt) => {
                return Ok(dt.format("%Y/%m/%d").to_string());
            }
            Err(err) => return Err(err),
        }
    }
    Ok(Local::now().format("%Y/%m/%d").to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
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

    let opt = CliOption {
        file: file,
        date: date,
    };

    let holidays = get_holidays(&opt.file)?;

    if holidays.contains_key(&opt.date) {
        let holiday = holidays.get(&opt.date).unwrap();
        println!("{} is holiday（{}）", opt.date, holiday);
    }

    Ok(())
}
