use std::{fs, io::Write};

use clap::Parser;
use foxy_utils::start_debug_logging_session;
use kon::{error::KonError, interpreter::Interpreter};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

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

    interpreter.show_tree();
  }

  Ok(())
}

fn run_prompt() -> Result<(), KonError> {
  let mut interpreter = Interpreter::new();

  let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
  let mut was_success = true;
  loop {
    // Print
    print_prompt(&mut out, was_success)?;

    // Read
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    // Evaluate
    if buffer.trim_end() == "#showast" {
      interpreter.show_tree();
      continue;
    }

    if buffer.trim_end() == "#showtokens" {
      interpreter.show_next_tokens();
      continue;
    }

    match interpreter.run(buffer) {
      Ok(result) => {
        was_success = true;
        writeln!(out, "{result}")?;
      }
      Err(error) => {
        was_success = false;
        out.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        writeln!(out, "{error}")?;
        out.reset()?;
      }
    };
  }
}

fn print_prompt(out: &mut StandardStream, was_success: bool) -> Result<(), KonError> {
  if was_success {
    out.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
  } else {
    out.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
  }
  write!(out, "\n$ ")?;
  out.reset()?;
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
