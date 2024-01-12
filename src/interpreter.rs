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
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Self { had_error: false }
    }

    pub fn run(&mut self, source_name: String, source: String) -> Result<(), KonError> {
        self.reset();

        let mut lexer = Lexer::new();

        match lexer.scan(&source_name, &source) {
            Ok(tokens) => {
                for token in tokens.iter() {
                    debug!("{token:?}");
                }
            },
            Err(errors) => return Err(KonError::InterpreterErrors(errors.to_vec())),
        };

        Ok(())
    }

    pub fn reset(&mut self) {
        self.had_error = false;
    }
}
