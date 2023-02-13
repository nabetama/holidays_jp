use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;
use chrono::NaiveDate;

#[cfg(test)]
mod tests {
    use crate::holiday::generator::generate;

    #[test]
    fn test_generate() {
        match generate() {
            Ok(it) => it,
            Err(err) => eprintln!("{}", err.to_string()),
        };

        assert_eq!(true, true);
    }
}

#[warn(dead_code)]
#[tokio::main]
pub async fn generate() -> Result<()> {
    let body = reqwest::get("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv")
        .await?
        .text_with_charset("shift-jis")
        .await?;

    let mut rdr = csv::Reader::from_reader(body.as_bytes());

    let header = "use std::collections::HashMap;
    pub fn dates() -> HashMap<String, String> {
        let res = HashMap::from([    
    ";

    let footer = "    ]);
    res
}";

    let path = Path::new("./src/holiday/dates.rs");
    let mut writer = BufWriter::new(File::create(&path)?);

    write!(&mut writer, "{}", header)?;
    for record in rdr.records() {
        let record = record?;

        let df = NaiveDate::parse_from_str(&String::from(&record[0]), "%Y/%m/%d")?;
        writeln!(
            &mut writer,
            "(\"{}\".to_string(), \"{}\".to_string()),",
            df.format("%Y/%m/%d"),
            &record[1]
        )?;
    }
    write!(&mut writer, "{}", footer)?;
    writer.flush()?;

    Ok(())
}
