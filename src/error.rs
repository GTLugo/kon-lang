use std::{fmt::Debug, io};

use strum::EnumDiscriminants;
use thiserror::Error;

pub mod error_handler;

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

#[derive(Error, Debug, EnumDiscriminants, Clone, PartialEq, Eq)]
pub enum InterpreterError {
  #[error("Unknown token `{token}` ({line}, {column})")]
  UnknownToken { line: u32, column: u32, token: String },
  #[error("{message} ({line}, {column})")]
  SyntaxError { line: u32, column: u32, message: String },
  #[error("Unterminated string ({line}, {column})")]
  UnterminatedString { line: u32, column: u32 },
  #[error("{message} ({line}, {column})")]
  ParseError { line: u32, column: u32, message: String },
  #[error("{message} ({line}, {column})")]
  EOFError { line: u32, column: u32, message: String },
  #[error("Expected `{delimiter}` or expression ({line}, {column})")]
  UnmatchedDelimiter { line: u32, column: u32, delimiter: String },
  #[error("Unspecified error")]
  Unspecified,
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
