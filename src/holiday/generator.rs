use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::{Result, Context};
use chrono::NaiveDate;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use httptest::{matchers::request, responders::status_code, Expectation, Server};
    use std::{fs::remove_file, io::prelude::*};

    #[test]
    fn test_generate() -> Result<()> {
        const OUT_FILE: &str = "dummy.rs";
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/dummy.csv")).respond_with(
                status_code(200).body("dummy-title,dummy-title2\r\n2022/01/01,HOLIDAY!\r\n"),
            ),
        );

        let url = server.url("/dummy.csv");
        generate(&url.to_string(), OUT_FILE)?;

        let mut f = File::open(OUT_FILE).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        assert!(contents.contains("\"2022-01-01\", \"HOLIDAY!\""));

        remove_file(OUT_FILE)?;

        Ok(())
    }
}

#[tokio::main]
pub async fn generate(url: &str, out_file: &str) -> Result<()> {
    let body = reqwest::get(url)
        .await
        .context("Failed to connect to the official holiday data source. Please check your internet connection.")?
        .text_with_charset("shift-jis")
        .await
        .context("Failed to download holiday data. The server might be temporarily unavailable.")?;

    let mut rdr = csv::Reader::from_reader(body.as_bytes());

    let header = "use std::collections::HashMap;
    pub fn dates() -> HashMap<&'static str, &'static str> {
        HashMap::from([    
    ";

    let footer = "    ])
}";

    let path = Path::new(&out_file);
    let mut writer = BufWriter::new(File::create(path)
        .context("Failed to create output file. Please check write permissions.")?);

    write!(&mut writer, "{header}")
        .context("Failed to write file header.")?;
    
    for record in rdr.records() {
        let record = record
            .context("Failed to parse CSV record. The data format might have changed.")?;

        let dt = NaiveDate::parse_from_str(&String::from(&record[0]), "%Y/%m/%d")
            .context("Failed to parse date from CSV. The date format in the source data might have changed.")?;
        writeln!(
            &mut writer,
            "(\"{}\", \"{}\"),",
            dt.format("%Y-%m-%d"),
            &record[1]
        )
        .context("Failed to write holiday data to file.")?;
    }
    write!(&mut writer, "{footer}")
        .context("Failed to write file footer.")?;
    writer.flush()
        .context("Failed to flush data to file. Data might be incomplete.")?;

    Ok(())
}
