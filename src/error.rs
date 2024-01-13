use std::{fmt::Debug, io};
use strum::EnumDiscriminants;
use thiserror::Error;

pub mod error_handler;

#[derive(Error, EnumDiscriminants)]
pub enum KonError {
    #[error("lexer caught {} error(s)", .0.len())]
    LexerErrors(Vec<InterpreterError>),
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("{0}")]
    Other(String),
    #[error("feature not implemented")]
    Unimplemented,
    #[error("unspecified interpreter error")]
    Unspecified,
}

impl Debug for KonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, EnumDiscriminants, Clone)]
pub enum InterpreterError {
    #[error("Unknown token `{token}` at {location} ({line}, {column})")]
    UnknownToken {
        line: u32,
        column: u32,
        location: String,
        token: String,
    },
    #[error("Syntax error `{message}` at {location} ({line}, {column})")]
    SyntaxError {
        line: u32,
        column: u32,
        location: String,
        message: String,
    },
    #[error("Unterminated string at {location} ({line}, {column})")]
    UnterminatedString {
        line: u32,
        column: u32,
        location: String,
    },
}

impl InterpreterError {
    pub fn id(&self) -> InterpreterErrorDiscriminants {
        self.into()
    }

    pub fn report(&self) {
        println!("{}", self.report_string());
    }

    pub fn report_string(&self) -> String {
        format!("{}", self)
    }
}
