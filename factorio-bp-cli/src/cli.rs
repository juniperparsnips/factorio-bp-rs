use core::str::FromStr;
use std::{io, path::PathBuf};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A utility tool for analyzing Factorio blueprints
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
/// Commands that the CLI can run
pub enum Command {
    /// Decode a blueprint string into JSON and its corresponding rust structure
    Decode(DecodeCommand),
}

#[derive(Args)]
/// Parameters needed for decoding a blueprint string
pub struct DecodeCommand {
    #[arg(short, long)]
    /// The path to the file containing the blueprint string
    pub infile: PathBuf,
    #[arg(short, long)]
    /// The path that the decoded blueprint should be written to
    pub outfile: PathBuf,
    #[arg(long)]
    /// The output file format
    pub outform: BpFormat,
    #[arg(short, long)]
    /// Verbosity of output
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A format that a blueprint output can be stored in
pub enum BpFormat {
    /// Javascript object notation
    JSON,
    /// Rust's debug print
    Rust,
}

impl FromStr for BpFormat {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(BpFormat::JSON),
            "rust" => Ok(BpFormat::Rust),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unexpected blueprint format '{s}'"),
            )),
        }
    }
}
