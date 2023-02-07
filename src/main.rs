use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Debug, Parser)]
struct Cli {
    file: String,
}

fn read_shift_jis_csv(path: &str) -> Result<(), Box<dyn Error>> {
    // 一度 Vec で読み込む
    let file = fs::read(path)?;
    // SHIFT_JIS を decode -> utf8 にencodeする
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file);

    let mut rdr = csv::Reader::from_reader(res.as_bytes());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    let args = Cli::parse();

    match read_shift_jis_csv(&args.file) {
        Ok(_) => {
            println!("done");
        }
        Err(err) => {
            println!("abnormal exit, {:?}", err.to_string())
        }
    }
}
