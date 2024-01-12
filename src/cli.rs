use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[clap(name = "Konpiler", author, version)]
pub struct Cli {
    #[clap(flatten)]
    pub mode: Mode,
    #[arg(short, long, conflicts_with("interactive"))]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct Mode {
    #[arg(short, long)]
    pub interactive: bool,
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}
