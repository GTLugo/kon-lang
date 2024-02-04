use foxy_utils::types::handle::Handle;

use self::{lexer::Lexer, parser::Parser};
use crate::error::{error_handler::ErrorHandler, KonError};

mod character_provider;
mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;
mod token_provider;

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
  pub fn new() -> Interpreter {
    Self
  }

  pub fn run(&mut self, source: String) -> Result<(), KonError> {
    let error_handler = Handle::new(ErrorHandler::new());

    let tokens = {
      let mut lexer = Lexer::new(error_handler.clone());
      lexer.scan(&source)
    };

    let expression = {
      let mut parser = Parser::new(error_handler.clone());
      parser.parse(&tokens)
    };

    error_handler.get().try_report_errors()?;

    println!("{expression}");

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
