use tracing::debug;

use crate::error::{KonError, error_handler::ErrorHandler};

use self::lexer::Lexer;

mod character_provider;
mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;

#[derive(Default)]
pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Interpreter {
        Self {}
    }

    pub fn run(&mut self, source_name: String, source: String) -> Result<(), KonError> {
        let mut error_handler = ErrorHandler::new();
        let mut lexer = Lexer::new();

        let tokens = lexer.scan(&source_name, &source, &mut error_handler);

        error_handler.try_report_errors()?;
        
        for token in tokens.iter() {
            debug!("{token:?}");
        }

        Ok(())
    }
}
