use std::{path::{PathBuf, Path}, fs::{File, self}, io::Write};

use clap::Parser;
use kon::compiler::Compiler;
use tracing::info;

mod log;

#[derive(Parser)]
#[command(name = "KON Compilier", author, version)]
struct Cli {
  #[arg(short, long)]
  input: PathBuf,
  #[arg(short, long)]
  output: Option<PathBuf>,
}

fn main() {
  log::init_max();

  const NAME: &str = env!("CARGO_PKG_NAME");
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  info!("{NAME}: {VERSION}");

  let cli = Cli::parse();
  let outpath = cli.output.unwrap_or_else(|| PathBuf::from("./a.out"));

  info!("Source: {:?}", cli.input);

  let compiler = Compiler::new();

  let result = compiler.compile(cli.input.clone()).and_then(|result| {
    fs::create_dir_all(outpath.parent().unwrap_or_else(|| Path::new("./"))).and_then(|_| {
      File::create(outpath.clone())
    }).and_then(|mut outfile| {
      outfile.write_all(&result)
    }).map_err(|err| err.into())
  });
  
  match result {
    Ok(_) => {
      println!("Kon Kon! Compilation success!");
    },
    Err(error) => {
      eprintln!("Kon Kon... Compilation failure: {}", error);
    },
  }
}
