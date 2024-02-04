use std::{fs, io::Write};

use clap::Parser;
use foxy_utils::start_debug_logging_session;
use kon::{error::KonError, interpreter::Interpreter};

use self::cli::Cli;

mod cli;

fn main() -> Result<(), KonError> {
  start_debug_logging_session!();

  let cli = Cli::parse();

  if cli.mode.interactive {
    run_prompt()
  } else {
    run_file(cli)
  }
}

fn run_file(flags: Cli) -> Result<(), KonError> {
  if let Some(file) = flags.mode.file {
    let mut interpreter = Interpreter::new();

    let source = fs::read_to_string(file)?;

    let result = interpreter.run(source)?;

    println!("{result}");
  }

  Ok(())
}

fn run_prompt() -> Result<(), KonError> {
  let mut interpreter = Interpreter::new();

  loop {
    // Print
    println!();
    print_prompt()?;

    // Read
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    // Evaluate
    if buffer.trim_end() == "#ast" {
      interpreter.show_tree();
      continue;
    }

    match interpreter.run(buffer) {
      Ok(result) => {
        println!("{result}");
      }
      Err(error) => {
        if let KonError::InterpreterErrors(..) = error {
          println!("{}", error);
        }
      }
    };
  }
}

fn print_prompt() -> Result<(), KonError> {
  print!("$ ");
  std::io::stdout().flush()?;

  Ok(())
}

// FOR COMPILER MODE:
// let outpath = flags.output.unwrap_or_else(|| PathBuf::from("./a.out"));
// let result = compile(file.clone()).and_then(|result| {
//     fs::create_dir_all(outpath.parent().unwrap_or_else(||
// Path::new("./"))).and_then(|_| {       File::create(outpath.clone())
//     }).and_then(|mut outfile| {
//       outfile.write_all(&result)
//     }).map_err(|err| err.into())
//   });
