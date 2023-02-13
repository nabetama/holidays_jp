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
    // csv ファイルをダウンロードする
    let body = reqwest::get("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv")
        .await?
        .text_with_charset("shift-jis")
        .await?;

    // csv 読み込む（csv reader そのまま使う）
    let mut rdr = csv::Reader::from_reader(body.as_bytes());

    // ファイルの先頭
    let header = "use std::collections::HashMap;
    pub fn dates() -> HashMap<String, String> {
        let res = HashMap::from([    
    ";

    let footer = "    ]);
    res
}";

    // csv から書き込む
    let path = Path::new("./src/holiday/dates.rs");

    println!("{:?}", path);

    // ファイルを書き込みモードで開く
    let mut writer = BufWriter::new(File::create(&path)?);

    // header 書き込む
    write!(&mut writer, "{}", header)?;

    // まんなか書き込む
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

    // footer 書き込む
    write!(&mut writer, "{}", footer)?;

    // flush する
    writer.flush()?;

    Ok(())
}
