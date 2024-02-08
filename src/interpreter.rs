use foxy_utils::types::handle::Handle;

use self::{grammar::syntax_tree::SyntaxTree, lexer::Lexer, parser::Parser};
use crate::error::{error_handler::ErrorHandler, KonError};

mod grammar;
mod lexer;
mod parser;
mod util;

pub struct Interpreter {
  error_handler: Handle<ErrorHandler>,
  lexer: Lexer,
  parser: Parser,
  show_tokens: bool,
  tree: Option<SyntaxTree>,
}

impl Default for Interpreter {
  fn default() -> Self {
    Self::new()
  }
}

impl Interpreter {
  pub fn new() -> Interpreter {
    let error_handler = Handle::new(ErrorHandler::new());
    let lexer = Lexer::new(error_handler.clone());
    let parser = Parser::new(error_handler.clone());

    Self {
      error_handler,
      lexer,
      parser,
      show_tokens: false,
      tree: None,
    }
  }

  pub fn run(&mut self, source: String) -> Result<String, KonError> {
    self.error_handler.get_mut().clear();
    let tokens = self.lexer.lex(&source);

    if self.show_tokens {
      println!("{tokens:#?}");
      self.show_tokens = false;
    }

    self.tree = Some(self.parser.parse(&tokens));

    self.error_handler.get().try_report_errors()?;

    // print!("Result: ");

    if let Ok(result) = self.tree.as_ref().unwrap().root.evaluate() {
      if let Some(&value) = result.downcast_ref::<i64>() {
        return Ok(value.to_string());
      }

      if let Some(value) = result.downcast_ref::<String>() {
        return Ok(value.clone());
      }
    }

    Err(KonError::Evaluation("invalid types".into()))
  }

  pub fn show_tree(&self) {
    if let Some(tree) = self.tree.as_ref() {
      print!("{}", tree); // tree has trailing newline due to recursive impl
    } else {
      println!("None");
    }
  }

  pub fn show_next_tokens(&mut self) {
    self.show_tokens = true;
  }
}
