use std::{fs, io::Write, ffi::OsStr};

use clap::Parser;
use kon::{error::KonError, interpreter::Interpreter};
use tracing::*;

use self::cli::Cli;

mod cli;
mod log;

fn main() -> Result<(), KonError> {
    log::init_max();

    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("{NAME}: {VERSION}");

    let cli = Cli::parse();

    let result = if cli.mode.interactive {
        run_prompt()
    } else {
        run_file(cli)
    };

    result.map_err(|error| {
        if let KonError::InterpreterErrors(errors) = &error {
            for error in errors {
                error.report();
            }
        }
        error
    })
}

fn run_file(flags: Cli) -> Result<(), KonError> {
    if let Some(file) = flags.mode.file {
        let name = file.file_name().unwrap_or(OsStr::new("file")).to_string_lossy().to_string();
        let mut interpreter = Interpreter::new();

        let source = fs::read_to_string(file)?;

        interpreter.run(name, source)?;
        // if let Err(error) = interpreter.run(source) {
        //     error.report();
        //     return Err(error);
        // }
    }

    Ok(())
}

fn run_prompt() -> Result<(), KonError> {
    let mut interpreter = Interpreter::new();

    print_prompt()?;
    loop {
        // Read
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        // Evaluate
        if let Err(KonError::InterpreterErrors(errors)) = interpreter.run("stdio".into(), buffer) {
            for error in &errors {
                error.report();
            }
            println!("{}", KonError::InterpreterErrors(errors)); // report errors without quitting for interactive mode
        };

        // Print
        println!();
        print_prompt()?;
    }
}

fn print_prompt() -> Result<(), KonError> {
    print!("> ");
    std::io::stdout().flush()?;

    Ok(())
}

// FOR COMPILER MODE:
// let outpath = flags.output.unwrap_or_else(|| PathBuf::from("./a.out"));
// let result = compile(file.clone()).and_then(|result| {
//     fs::create_dir_all(outpath.parent().unwrap_or_else(|| Path::new("./"))).and_then(|_| {
//       File::create(outpath.clone())
//     }).and_then(|mut outfile| {
//       outfile.write_all(&result)
//     }).map_err(|err| err.into())
//   });
