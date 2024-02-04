use foxy_utils::types::handle::Handle;

use self::{lexer::Lexer, parser::Parser};
use crate::error::{error_handler::ErrorHandler, KonError};

mod character_provider;
mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;
mod token_provider;

pub struct Interpreter {
  error_handler: Handle<ErrorHandler>,
  lexer: Lexer,
  parser: Parser,
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
    }
  }

  pub fn run(&mut self, source: String) -> Result<(), KonError> {
    let tokens = self.lexer.scan(&source);

    let expression = self.parser.parse(&tokens);

    println!("{expression}");

    self.error_handler.get().try_report_errors()?;
    self.error_handler.get_mut().clear();

    print!("Result: ");
    if let Ok(result) = expression.root.evaluate() {
      if let Some(&value) = result.downcast_ref::<f64>() {
        println!("{value}");
      }

      if let Some(value) = result.downcast_ref::<String>() {
        println!("{value}");
      }
    }

    Ok(())
  }
}
