use std::{fs, io::Write};

use clap::Parser;
use kon::{error::KonError, interpreter::Interpreter};

use self::cli::Cli;

mod cli;
mod log;

fn main() -> Result<(), KonError> {
    log::init_max_debug();

    let cli = Cli::parse();

    if cli.mode.interactive {
        run_prompt()
    } else {
        run_file(cli)
    }
}

fn run_file(flags: Cli) -> Result<(), KonError> {
    if let Some(file) = flags.mode.file {
        // let name = file
        //     .file_name()
        //     .unwrap_or(std::ffi::OsStr::new("file"))
        //     .to_string_lossy()
        //     .to_string();
        let mut interpreter = Interpreter::new();

        let source = fs::read_to_string(file)?;

        interpreter.run(source)?;
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
        if let Err(error) = interpreter.run(buffer) {
            if let KonError::InterpreterErrors(..) = error {
                println!("{}", error);
            }
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
