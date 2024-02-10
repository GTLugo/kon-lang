use std::{fmt::Debug, io};

use strum::EnumDiscriminants;
use thiserror::Error;

use super::grammar::token::Position;

pub mod error_handler;

#[derive(Error, EnumDiscriminants)]
pub enum KonError {
  #[error("interpreter caught {} error(s)", .0.len())]
  InterpreterErrors(Vec<InterpreterError>),
  #[error("failed to evaluate expression: `{0}`")]
  Evaluation(String),
  #[error("{0}")]
  IOError(#[from] io::Error),
  #[error("{0}")]
  Other(String),
}

impl Debug for KonError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Error, Debug, EnumDiscriminants, Clone, PartialEq, Eq)]
pub enum InterpreterError {
  #[error("Unknown token `{token}` {position}")]
  UnknownToken { position: Position, token: String },
  #[error("{message} {position}")]
  SyntaxError { position: Position, message: String },
  #[error("Unterminated string {position}")]
  UnterminatedString { position: Position },
  #[error("{message} {position}")]
  ParseError { position: Position, message: String },
  #[error("{message} {position}")]
  EOFError { position: Position, message: String },
  #[error("Unmatched `{delimiter}` {position}")]
  UnmatchedDelimiter { position: Position, delimiter: String },
  #[error("Unknown operator `{operator}` {position}")]
  UnknownOperator { position: Position, operator: String },
  #[error("{0}")]
  Other(String),
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
