use super::{InterpreterError, KonError};

pub struct ErrorHandler {
  errors: Vec<InterpreterError>,
}

impl Default for ErrorHandler {
  fn default() -> Self {
    Self::new()
  }
}

impl ErrorHandler {
  pub fn new() -> Self {
    Self {
      errors: Default::default(),
    }
  }

  pub fn push(&mut self, error: InterpreterError) {
    self.errors.push(error.clone());
  }

  pub fn had_error(&self) -> bool {
    !self.errors.is_empty()
  }

  pub fn errors(&self) -> &[InterpreterError] {
    &self.errors
  }

  pub fn clear(&mut self) {
    self.errors.clear();
  }

  pub fn report_errors(&self) -> KonError {
    for error in &self.errors {
      error.report();
    }
    return KonError::InterpreterErrors(self.errors().to_vec());
  }

  pub fn try_report_errors(&self) -> Result<(), KonError> {
    if self.had_error() {
      Err(self.report_errors())
    } else {
      Ok(())
    }
  }
}
