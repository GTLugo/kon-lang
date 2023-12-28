use std::fmt::{Display, Formatter};

use crate::error::KonError;

#[derive(Debug)]
pub enum Token {
  // Single-character
  LeftParenthesis      { line: u32, column: u32 },
  RightParenthesis     { line: u32, column: u32 },
  LeftAngledBracket    { line: u32, column: u32 },
  RightAngledBracket   { line: u32, column: u32 },
  LeftSquareBracket    { line: u32, column: u32 },
  RightSquareBracket   { line: u32, column: u32 },
  LeftSquigglyBracket  { line: u32, column: u32 },
  RightSquigglyBracket { line: u32, column: u32 },
  Hashtag              { line: u32, column: u32 },
  Comma                { line: u32, column: u32 },
  Period               { line: u32, column: u32 },
  Colon                { line: u32, column: u32 },
  Semicolon            { line: u32, column: u32 },
  QuotationMark        { line: u32, column: u32 },
  Apostrophe           { line: u32, column: u32 },

  // One or two characters
  Plus                     { line: u32, column: u32 },
  Minus                    { line: u32, column: u32 },
  Asterisk                 { line: u32, column: u32 },
  ForwardSlash             { line: u32, column: u32 },
  ExclamationPoint         { line: u32, column: u32 },
  LessThan                 { line: u32, column: u32 },
  GreaterThan              { line: u32, column: u32 },
  Equals                   { line: u32, column: u32 },
  PlusEquals               { line: u32, column: u32 },
  MinusEquals              { line: u32, column: u32 },
  AsteriskEquals           { line: u32, column: u32 },
  ForwardSlashEquals       { line: u32, column: u32 },
  ExclamationPointEquals   { line: u32, column: u32 },
  LessThanEquals           { line: u32, column: u32 },
  GreaterThanEquals        { line: u32, column: u32 },
  RightArrow               { line: u32, column: u32 },

  // Literals
  Identifier { line: u32, column: u32, lexeme: String },
  String     { line: u32, column: u32, lexeme: String },
  Number     { line: u32, column: u32, lexeme: f64 },
  Bool       { line: u32, column: u32, lexeme: bool },

  // Keywords
  True   { line: u32, column: u32 },
  False  { line: u32, column: u32 },
  If     { line: u32, column: u32 },
  Else   { line: u32, column: u32 },
  For    { line: u32, column: u32 },
  While  { line: u32, column: u32 },
  Loop   { line: u32, column: u32 },
  // Fn     { line: u32 },
  Return { line: u32, column: u32 },
  Null   { line: u32, column: u32 },
  This   { line: u32, column: u32 },
  Super  { line: u32, column: u32 },
  // Var    { line: u32 },
  Use    { line: u32, column: u32 },
  Struct { line: u32, column: u32 },
  Impl   { line: u32, column: u32 },
  As     { line: u32, column: u32 },

  EndOfFile { line: u32, column: u32 },

  Invalid { error: KonError }
}

impl Token {
  pub fn line(&self) -> u32 {
    match self {
      Token::LeftParenthesis        { line, .. } => *line,
      Token::RightParenthesis       { line, .. } => *line,
      Token::LeftAngledBracket      { line, .. } => *line,
      Token::RightAngledBracket     { line, .. } => *line,
      Token::LeftSquareBracket      { line, .. } => *line,
      Token::RightSquareBracket     { line, .. } => *line,
      Token::LeftSquigglyBracket    { line, .. } => *line,
      Token::RightSquigglyBracket   { line, .. } => *line,
      Token::Hashtag                { line, .. } => *line,
      Token::Comma                  { line, .. } => *line,
      Token::Period                 { line, .. } => *line,
      Token::Colon                  { line, .. } => *line,
      Token::Semicolon              { line, .. } => *line,
      Token::QuotationMark          { line, .. } => *line,
      Token::Apostrophe             { line, .. } => *line,

      Token::Plus                   { line, .. } => *line,
      Token::Minus                  { line, .. } => *line,
      Token::Asterisk               { line, .. } => *line,
      Token::ForwardSlash           { line, .. } => *line,
      Token::ExclamationPoint       { line, .. } => *line,
      Token::LessThan               { line, .. } => *line,
      Token::GreaterThan            { line, .. } => *line,
      Token::Equals                 { line, .. } => *line,
      Token::PlusEquals             { line, .. } => *line,
      Token::MinusEquals            { line, .. } => *line,
      Token::AsteriskEquals         { line, .. } => *line,
      Token::ForwardSlashEquals     { line, .. } => *line,
      Token::ExclamationPointEquals { line, .. } => *line,
      Token::LessThanEquals         { line, .. } => *line,
      Token::GreaterThanEquals      { line, .. } => *line,
      Token::RightArrow             { line, .. } => *line,

      Token::Identifier             { line, .. } => *line,
      Token::String                 { line, .. } => *line,
      Token::Number                 { line, .. } => *line,
      Token::Bool                   { line, .. } => *line,

      Token::True                   { line, .. } => *line,
      Token::False                  { line, .. } => *line,
      Token::If                     { line, .. } => *line,
      Token::Else                   { line, .. } => *line,
      Token::For                    { line, .. } => *line,
      Token::While                  { line, .. } => *line,
      Token::Loop                   { line, .. } => *line,
      Token::Return                 { line, .. } => *line,
      Token::Null                   { line, .. } => *line,
      Token::This                   { line, .. } => *line,
      Token::Super                  { line, .. } => *line,
      Token::Use                    { line, .. } => *line,
      Token::Struct                 { line, .. } => *line,
      Token::Impl                   { line, .. } => *line,
      Token::As                     { line, .. } => *line,

      Token::EndOfFile              { line, .. } => *line,

      Token::Invalid { .. } => 0,
    }
  } 

  pub fn column(&self) -> u32 {
    match self {
      Token::LeftParenthesis        { column, .. } => *column,
      Token::RightParenthesis       { column, .. } => *column,
      Token::LeftAngledBracket      { column, .. } => *column,
      Token::RightAngledBracket     { column, .. } => *column,
      Token::LeftSquareBracket      { column, .. } => *column,
      Token::RightSquareBracket     { column, .. } => *column,
      Token::LeftSquigglyBracket    { column, .. } => *column,
      Token::RightSquigglyBracket   { column, .. } => *column,
      Token::Hashtag                { column, .. } => *column,
      Token::Comma                  { column, .. } => *column,
      Token::Period                 { column, .. } => *column,
      Token::Colon                  { column, .. } => *column,
      Token::Semicolon              { column, .. } => *column,
      Token::QuotationMark          { column, .. } => *column,
      Token::Apostrophe             { column, .. } => *column,

      Token::Plus                   { column, .. } => *column,
      Token::Minus                  { column, .. } => *column,
      Token::Asterisk               { column, .. } => *column,
      Token::ForwardSlash           { column, .. } => *column,
      Token::ExclamationPoint       { column, .. } => *column,
      Token::LessThan               { column, .. } => *column,
      Token::GreaterThan            { column, .. } => *column,
      Token::Equals                 { column, .. } => *column,
      Token::PlusEquals             { column, .. } => *column,
      Token::MinusEquals            { column, .. } => *column,
      Token::AsteriskEquals         { column, .. } => *column,
      Token::ForwardSlashEquals     { column, .. } => *column,
      Token::ExclamationPointEquals { column, .. } => *column,
      Token::LessThanEquals         { column, .. } => *column,
      Token::GreaterThanEquals      { column, .. } => *column,
      Token::RightArrow             { column, .. } => *column,

      Token::Identifier             { column, .. } => *column,
      Token::String                 { column, .. } => *column,
      Token::Number                 { column, .. } => *column,
      Token::Bool                   { column, .. } => *column,

      Token::True                   { column, .. } => *column,
      Token::False                  { column, .. } => *column,
      Token::If                     { column, .. } => *column,
      Token::Else                   { column, .. } => *column,
      Token::For                    { column, .. } => *column,
      Token::While                  { column, .. } => *column,
      Token::Loop                   { column, .. } => *column,
      Token::Return                 { column, .. } => *column,
      Token::Null                   { column, .. } => *column,
      Token::This                   { column, .. } => *column,
      Token::Super                  { column, .. } => *column,
      Token::Use                    { column, .. } => *column,
      Token::Struct                 { column, .. } => *column,
      Token::Impl                   { column, .. } => *column,
      Token::As                     { column, .. } => *column,

      Token::EndOfFile              { column, .. } => *column,

      Token::Invalid { .. } => 0,
    }
  } 

  pub fn lexeme(&self) -> String {
    match self {
      Token::LeftParenthesis        { .. } => "(".into(),
      Token::RightParenthesis       { .. } => ")".into(),
      Token::LeftAngledBracket      { .. } => "<".into(),
      Token::RightAngledBracket     { .. } => ">".into(),
      Token::LeftSquareBracket      { .. } => "[".into(),
      Token::RightSquareBracket     { .. } => "]".into(),
      Token::LeftSquigglyBracket    { .. } => "{".into(),
      Token::RightSquigglyBracket   { .. } => "}".into(),
      Token::Hashtag                { .. } => "#".into(),
      Token::Comma                  { .. } => ",".into(),
      Token::Period                 { .. } => ".".into(),
      Token::Colon                  { .. } => ":".into(),
      Token::Semicolon              { .. } => ";".into(),
      Token::QuotationMark          { .. } => "\"".into(),
      Token::Apostrophe             { .. } => "\'".into(),

      Token::Plus                   { .. } => "+".into(),
      Token::Minus                  { .. } => "-".into(),
      Token::Asterisk               { .. } => "*".into(),
      Token::ForwardSlash           { .. } => "/".into(),
      Token::ExclamationPoint       { .. } => "!".into(),
      Token::LessThan               { .. } => "<".into(),
      Token::GreaterThan            { .. } => ">".into(),
      Token::Equals                 { .. } => "=".into(),
      Token::PlusEquals             { .. } => "+=".into(),
      Token::MinusEquals            { .. } => "-=".into(),
      Token::AsteriskEquals         { .. } => "*=".into(),
      Token::ForwardSlashEquals     { .. } => "/=".into(),
      Token::ExclamationPointEquals { .. } => "!=".into(),
      Token::LessThanEquals         { .. } => "<=".into(),
      Token::GreaterThanEquals      { .. } => ">=".into(),
      Token::RightArrow             { .. } => "->".into(),

      Token::Identifier             { lexeme, .. } => lexeme.clone(),
      Token::String                 { lexeme, .. } => lexeme.clone(),
      Token::Number                 { lexeme, .. } => lexeme.to_string(),
      Token::Bool                   { lexeme, .. } => lexeme.to_string(),

      Token::True                   { .. } => "true".into(),
      Token::False                  { .. } => "false".into(),
      Token::If                     { .. } => "if".into(),
      Token::Else                   { .. } => "else".into(),
      Token::For                    { .. } => "for".into(),
      Token::While                  { .. } => "while".into(),
      Token::Loop                   { .. } => "loop".into(),
      Token::Return                 { .. } => "return".into(),
      Token::Null                   { .. } => "null".into(),
      Token::This                   { .. } => "this".into(),
      Token::Super                  { .. } => "super".into(),
      Token::Use                    { .. } => "use".into(),
      Token::Struct                 { .. } => "struct".into(),
      Token::Impl                   { .. } => "impl".into(),
      Token::As                     { .. } => "as".into(),

      Token::EndOfFile              { .. } => "[EOF]".into(),

      Token::Invalid { .. } => "[INV]".into(),
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.lexeme())
  }
}