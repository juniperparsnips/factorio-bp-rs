use std::{fs, io::Read};

use base64::{engine::general_purpose, Engine};
use clap::Parser;
use factorio_bp_rs::blueprint::{Blueprint, BlueprintBook};
use flate2::read::ZlibDecoder;
use serde_json::{Map, Value};

mod cli;

use self::cli::{BpFormat, Cli, Command, DecodeCommand};

fn decode_bp(args: &DecodeCommand) -> Result<(), std::io::Error> {
    let mut input = fs::read_to_string(&args.infile)?;

    // Remove all line breaks before parsing
    input.retain(|c| !(c == '\n' || c == '\r'));

    // Remove the "version byte"
    let mut input_iter = input.chars();
    let _version = input_iter.next().unwrap();

    let parseable: String = input_iter.collect();

    let input_bytes = general_purpose::STANDARD
        .decode(parseable)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut z = ZlibDecoder::new(&input_bytes[..]);
    let mut json = String::new();
    z.read_to_string(&mut json)?;

    let data_to_write = match args.outform {
        BpFormat::JSON => json,
        BpFormat::Rust => {
            // Determine if its a bp book or single bp
            let v: Map<String, Value> = serde_json::from_str(&json)?;

            if let Some(inner) = v.get("blueprint") {
                let js = &inner.to_string();
                // println!("{}", js);
                let bp: Blueprint = serde_json::from_str(&inner.to_string())?;
                format!("{:?}", bp)
            } else if let Some(inner) = v.get("blueprint_book") {
                let bp_book: BlueprintBook = serde_json::from_str(&inner.to_string())?;
                format!("{:?}", bp_book)
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("given data is not a blueprint or blueprint book"),
                ));
            }
        }
    };

    fs::write(&args.outfile, data_to_write)?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Decode(args) => {
            decode_bp(args)?;
        }
    }

    Ok(())
}
