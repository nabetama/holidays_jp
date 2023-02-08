use chrono::NaiveDate;
use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Debug, Parser)]
struct Cli {
    file: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Holiday {
    date: NaiveDate,
    name: String,
}

fn read_shift_jis_csv(path: &str) -> Result<Vec<Holiday>, Box<dyn Error>> {
    // 一度 Vec で読み込む
    let file = fs::read(path)?;
    // SHIFT_JIS を decode -> utf8 にencodeする
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
    let mut rdr = csv::Reader::from_reader(res.as_bytes());

    let mut holidays: Vec<Holiday> = Vec::new();
    for record in rdr.records() {
        let record = record?;

        let holiday = Holiday {
            date: NaiveDate::parse_from_str(&String::from(&record[0]), "%Y/%m/%d")?,
            name: String::from(&record[1]),
        };

        holidays.push(holiday);
    }

    Ok(holidays)
}

fn main() {
    let args = Cli::parse();

    match read_shift_jis_csv(&args.file) {
        Ok(holidays) => {
            for holiday in holidays {
                println!("{:?}", holiday);
            }
        }
        Err(err) => {
            println!("abnormal exit, {:?}", err.to_string())
        }
    }
}
