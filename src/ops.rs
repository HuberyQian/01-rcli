use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author,about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show Csv orConvert CSV to Other formats")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_file_exists(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File {} does not exist".into())
    }
}
