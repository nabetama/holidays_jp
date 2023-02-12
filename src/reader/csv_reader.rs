use anyhow::Result;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;

#[test]
fn test_get_holidays() {
    let holidays = get_holidays("assets/syukujitsu.csv").unwrap();

    assert_eq!(holidays.contains_key("20230101"), true);
}

pub fn get_holidays(path: &str) -> Result<HashMap<String, String>> {
    let mut dates = HashMap::new();

    // 一度 Vec で読み込む
    let file = fs::read(path)?;
    // SHIFT_JIS を decode -> utf8 にencodeする
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
    let mut rdr = csv::Reader::from_reader(res.as_bytes());

    for record in rdr.records() {
        let record = record?;

        let df = NaiveDate::parse_from_str(&String::from(&record[0]), "%Y/%m/%d")?;
        dates.insert(df.format("%Y%m%d").to_string(), String::from(&record[1]));
    }

    Ok(dates)
}
