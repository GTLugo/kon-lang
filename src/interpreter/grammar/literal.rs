use std::any::Any;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  // Literals
  Identifier { lexeme: String },
  String { lexeme: String },
  Number { lexeme: i64 },
  Void,
}

impl Literal {
  pub const VOID: &'static str = "()";

  pub fn lexeme(&self) -> String {
    match self {
      Literal::Identifier { lexeme, .. } => lexeme.clone(),
      Literal::String { lexeme, .. } => lexeme.clone(),
      Literal::Number { lexeme, .. } => lexeme.to_string(),
      Literal::Void => Self::VOID.into(),
    }
  }

  pub fn value(&self) -> Box<dyn Any> {
    match self.clone() {
      Literal::Identifier { lexeme } => Box::new(lexeme),
      Literal::String { lexeme } => Box::new(lexeme),
      Literal::Number { lexeme } => Box::new(lexeme),
      Literal::Void => Box::new(()),
    }
  }
}
