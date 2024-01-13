use crate::interpreter::token::Token;

use super::LexerError;

pub struct ErrorHandler {
    errors: Vec<LexerError>,
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            errors: Default::default()
        }
    }

    pub fn lexing_error(&mut self, error: LexerError) -> Option<Token> {
        self.errors.push(error.clone());

        Some(Token::Invalid { error })
    }

    pub fn parsing_error(&mut self, error: LexerError) -> Token {
        self.errors.push(error.clone());

        Token::Invalid { error }
    }

    pub fn had_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn errors(&self) -> &[LexerError] {
        &self.errors
    }

    pub fn report_errors(&self) {
        for error in &self.errors {
            error.report();
        }
    }
}