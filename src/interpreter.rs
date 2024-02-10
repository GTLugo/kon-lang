use foxy_utils::types::handle::Handle;

use self::{
  error::{error_handler::ErrorHandler, KonError},
  grammar::binding::binder::Binder,
  lexer::Lexer,
  parser::Parser,
};

pub mod error;
mod grammar;
mod lexer;
mod parser;
mod util;

pub struct Interpreter {
  error_handler: Handle<ErrorHandler>,

  lexer: Lexer,
  parser: Parser,
  binder: Binder,

  show_tokens: bool,
  show_tree: bool,
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
    let binder = Binder::new(error_handler.clone());

    Self {
      error_handler,
      lexer,
      parser,
      binder,
      show_tokens: false,
      show_tree: false,
    }
  }

  pub fn run(&mut self, source: String) -> Result<String, KonError> {
    self.error_handler.get_mut().clear();
    let tokens = self.lexer.lex(&source);

    if self.show_tokens {
      println!("{tokens:#?}");
      self.show_tokens = false;
    }

    let tree = self.parser.parse(&tokens);

    if self.show_tree {
      print!("{}", tree); // tree has trailing newline due to recursive impl
      self.show_tree = false;
    }

    let bound_tree = self.binder.bind(tree.root);

    self.error_handler.get().try_report_errors()?;

    // print!("Result: ");

    if let Ok(result) = bound_tree.evaluate() {
      if let Some(&value) = result.downcast_ref::<i64>() {
        return Ok(value.to_string());
      }

      if let Some(value) = result.downcast_ref::<String>() {
        return Ok(value.clone());
      }
    }

    Err(KonError::Evaluation("invalid types".into()))
  }

  pub fn show_tree(&mut self) {
    self.show_tree = true;
  }

  pub fn show_next_tokens(&mut self) {
    self.show_tokens = true;
  }
}
