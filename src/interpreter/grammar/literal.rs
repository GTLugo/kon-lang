#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  // Literals
  Identifier { lexeme: String },
  String { lexeme: String },
  Number { lexeme: i64 },
  Void,
}

impl Literal {
  pub fn lexeme(&self) -> String {
    match self {
      Literal::Identifier { lexeme, .. } => lexeme.clone(),
      Literal::String { lexeme, .. } => lexeme.clone(),
      Literal::Number { lexeme, .. } => lexeme.to_string(),
      Literal::Void => "()".into(),
    }
  }
}
