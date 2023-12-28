use std::fs;
use std::path::Path;

use tracing::trace;

use crate::compiler::lexer::Lexer;
use crate::error::KonError;

mod character_provider;
mod parser;
mod lexer;
mod token;

#[derive(Default)]
pub struct Compiler {
  
}

impl Compiler {
  pub fn new() -> Compiler {

    Self {
      
    }
  }

  pub fn compile<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<u8>, KonError> {
    let source = fs::read_to_string(file_path)?;
    let lexer = Lexer::new(&source);

    for token in lexer {
      trace!("{token:?}");
    }

    Err(KonError::Unimplemented)
  }
}
