use clap::Parser;
use common::constants::BIN_NAME;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = BIN_NAME, author, version,  about)]
pub struct Cli {
    pub path: Option<PathBuf>,
}
