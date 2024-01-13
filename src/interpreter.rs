use tracing::debug;

use crate::error::KonError;

use self::lexer::Lexer;

mod character_provider;
mod expression;
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
        let mut lexer = Lexer::new();
        
        match lexer.scan(&source_name, &source) {
            Ok(tokens) => {
                for token in tokens.iter() {
                    debug!("{token:?}");
                }
            },
            Err(error_handler) => {
                error_handler.report_errors();
                return Err(KonError::InterpreterErrors(error_handler));
            },
        };

        Ok(())
    }
}
