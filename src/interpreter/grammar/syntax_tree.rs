use std::fmt::Display;

use super::{expression::Expression, token::Token};

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
  pub root: Expression,
  pub eof: Token,
}

impl Display for SyntaxTree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.root.pretty_print(0, f)
  }
}
