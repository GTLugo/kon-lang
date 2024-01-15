use crate::error::{error_handler::ErrorHandler, KonError};

use self::{lexer::Lexer, parser::Parser};

mod character_provider;
mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;
mod token_provider;

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Self {}
    }

    pub fn run(&mut self, source: String) -> Result<(), KonError> {
        let mut error_handler = ErrorHandler::new();
        let mut lexer = Lexer::new();

        let tokens = lexer.scan(&source, &mut error_handler);

        // for token in &tokens {
        //     tracing::debug!("{token:?}");
        // }

        let mut parser = Parser::new(&mut error_handler);

        let expression = parser.parse(&tokens);

        println!("{expression:#?}");

        error_handler.try_report_errors()?;

        Ok(())
    }
}
