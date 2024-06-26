use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ops::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;

    let headers = rdr.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(output, json)?;
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(&ret)?;
            fs::write(output, yaml)?;
        }
    }

    Ok(())
}
