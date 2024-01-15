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

        let tokens = {
            let mut lexer = Lexer::new();
            lexer.scan(&source, &mut error_handler)
        };

        // for token in &tokens {
        //     tracing::debug!("{token:?}");
        // }

        let expression = {
            let mut parser = Parser::new(&mut error_handler);
            parser.parse(&tokens)
        };

        // println!("{expression:#?}");

        error_handler.try_report_errors()?;

        if let Ok(result) = expression.evaluate() {
            if let Some(&value) = result.downcast_ref::<f64>() {
                println!("{value}");
            }

            if let Some(value) = result.downcast_ref::<String>() {
                println!("{value}");
            }
        } else {
            println!("Invalid");
        }

        Ok(())
    }
}
