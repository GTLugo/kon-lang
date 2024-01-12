use std::{fmt::Debug, io};
use strum::EnumDiscriminants;
use thiserror::Error;

#[derive(Error, EnumDiscriminants)]
pub enum KonError {
    #[error("interpreter caught {} error(s)", .0.len())]
    InterpreterErrors(Vec<InterpreterError>),
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
        // match self {
        //     InterpreterError::UnknownToken {
        //         location,
        //         line,
        //         column,
        //         token,
        //     } => format!("{:?} at {location}({line}:{column}) `{token}`", self.id()),
        //     InterpreterError::SyntaxError {
        //         location,
        //         line,
        //         column,
        //         message,
        //     } => format!(
        //         "{:?} at {location} ({line}:{column}) `{message}`",
        //         self.id()
        //     ),
        //     InterpreterError::UnterminatedString { line, column, location } => format!(
        //         "{:?} at {location} ({line}:{column})",
        //         self.id()
        //     ),
        // }
    }
}
