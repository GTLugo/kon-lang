use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KonError {
  #[error("script `{0}` was not found")]
  IOError(#[from] io::Error),
  #[error("{location}: {line} | Unknown Token: `{token}`")]
  UnknownToken { location: String, line: u32, column: u32, token: String },
  #[error("{location}: {line} | Syntax Error: `{message}`")]
  SyntaxError { location: String, line: u32, column: u32, message: String },
  #[error("feature not implemented")]
  Unimplemented,
  #[error("{0}")]
  Other(String),
  #[error("unspecified interpreter error")]
  Unspecified,
}
