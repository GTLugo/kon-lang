use std::fmt::{Display, Formatter};

use strum::EnumDiscriminants;

use crate::error::InterpreterError;

#[derive(Debug, EnumDiscriminants)]
pub enum Token {
    // Single-character
    LeftParenthesis {
        line: u32,
        column: u32,
    },
    RightParenthesis {
        line: u32,
        column: u32,
    },
    LeftAngledBracket {
        line: u32,
        column: u32,
    },
    RightAngledBracket {
        line: u32,
        column: u32,
    },
    LeftSquareBracket {
        line: u32,
        column: u32,
    },
    RightSquareBracket {
        line: u32,
        column: u32,
    },
    LeftSquigglyBracket {
        line: u32,
        column: u32,
    },
    RightSquigglyBracket {
        line: u32,
        column: u32,
    },
    Hashtag {
        line: u32,
        column: u32,
    },
    Comma {
        line: u32,
        column: u32,
    },
    Period {
        line: u32,
        column: u32,
    },
    Colon {
        line: u32,
        column: u32,
    },
    Semicolon {
        line: u32,
        column: u32,
    },
    QuotationMark {
        line: u32,
        column: u32,
    },
    Apostrophe {
        line: u32,
        column: u32,
    },

    // One or two characters
    Plus {
        line: u32,
        column: u32,
    },
    Minus {
        line: u32,
        column: u32,
    },
    Asterisk {
        line: u32,
        column: u32,
    },
    ForwardSlash {
        line: u32,
        column: u32,
    },
    ExclamationPoint {
        line: u32,
        column: u32,
    },
    LessThan {
        line: u32,
        column: u32,
    },
    GreaterThan {
        line: u32,
        column: u32,
    },
    Equals {
        line: u32,
        column: u32,
    },
    DoubleEquals {
        line: u32,
        column: u32,
    },
    PlusEquals {
        line: u32,
        column: u32,
    },
    MinusEquals {
        line: u32,
        column: u32,
    },
    AsteriskEquals {
        line: u32,
        column: u32,
    },
    ForwardSlashEquals {
        line: u32,
        column: u32,
    },
    ExclamationPointEquals {
        line: u32,
        column: u32,
    },
    LessThanEquals {
        line: u32,
        column: u32,
    },
    GreaterThanEquals {
        line: u32,
        column: u32,
    },
    RightArrow {
        line: u32,
        column: u32,
    },
    Ampersand {
        line: u32,
        column: u32,
    },
    DoubleAmpersand {
        line: u32,
        column: u32,
    },
    AmpersandEquals {
        line: u32,
        column: u32,
    },
    Pipe {
        line: u32,
        column: u32,
    },
    DoublePipe {
        line: u32,
        column: u32,
    },
    PipeEquals {
        line: u32,
        column: u32,
    },
    Caret {
        line: u32,
        column: u32,
    },
    CaretEquals {
        line: u32,
        column: u32,
    },
    Tilde {
        line: u32,
        column: u32,
    },
    TildeEquals {
        line: u32,
        column: u32,
    },
    Percent {
        line: u32,
        column: u32,
    },
    PercentEquals {
        line: u32,
        column: u32,
    },

    // Literals
    Identifier {
        line: u32,
        column: u32,
        lexeme: String,
    },
    String {
        line: u32,
        column: u32,
        lexeme: String,
    },
    Number {
        line: u32,
        column: u32,
        lexeme: f64,
    },
    // Bool       { line: u32, column: u32, lexeme: bool }, // use enum

    // Keywords
    // True   { line: u32, column: u32 },
    // False  { line: u32, column: u32 },
    If {
        line: u32,
        column: u32,
    },
    Else {
        line: u32,
        column: u32,
    },
    For {
        line: u32,
        column: u32,
    },
    While {
        line: u32,
        column: u32,
    },
    Loop {
        line: u32,
        column: u32,
    },
    // Fn     { line: u32 },
    Return {
        line: u32,
        column: u32,
    },
    // Null {
    //     line: u32,
    //     column: u32,
    // },
    Selff {
        line: u32,
        column: u32,
    },
    Super {
        line: u32,
        column: u32,
    },
    // Var    { line: u32 },
    Use {
        line: u32,
        column: u32,
    },
    Struct {
        line: u32,
        column: u32,
    },
    Impl {
        line: u32,
        column: u32,
    },
    As {
        line: u32,
        column: u32,
    },

    EndOfFile {
        line: u32,
        column: u32,
    },

    Invalid {
        error: InterpreterError,
    },
}

impl Token {
    pub fn id(&self) -> TokenDiscriminants {
        self.into()
    }

    pub fn line(&self) -> u32 {
        match self {
            Token::LeftParenthesis { line, .. } => *line,
            Token::RightParenthesis { line, .. } => *line,
            Token::LeftAngledBracket { line, .. } => *line,
            Token::RightAngledBracket { line, .. } => *line,
            Token::LeftSquareBracket { line, .. } => *line,
            Token::RightSquareBracket { line, .. } => *line,
            Token::LeftSquigglyBracket { line, .. } => *line,
            Token::RightSquigglyBracket { line, .. } => *line,
            Token::Hashtag { line, .. } => *line,
            Token::Comma { line, .. } => *line,
            Token::Period { line, .. } => *line,
            Token::Colon { line, .. } => *line,
            Token::Semicolon { line, .. } => *line,
            Token::QuotationMark { line, .. } => *line,
            Token::Apostrophe { line, .. } => *line,

            Token::Plus { line, .. } => *line,
            Token::Minus { line, .. } => *line,
            Token::Asterisk { line, .. } => *line,
            Token::ForwardSlash { line, .. } => *line,
            Token::ExclamationPoint { line, .. } => *line,
            Token::LessThan { line, .. } => *line,
            Token::GreaterThan { line, .. } => *line,
            Token::Equals { line, .. } => *line,
            Token::DoubleEquals { line, .. } => *line,
            Token::PlusEquals { line, .. } => *line,
            Token::MinusEquals { line, .. } => *line,
            Token::AsteriskEquals { line, .. } => *line,
            Token::ForwardSlashEquals { line, .. } => *line,
            Token::ExclamationPointEquals { line, .. } => *line,
            Token::LessThanEquals { line, .. } => *line,
            Token::GreaterThanEquals { line, .. } => *line,
            Token::RightArrow { line, .. } => *line,
            Token::Ampersand { line, .. } => *line,
            Token::DoubleAmpersand { line, .. } => *line,
            Token::AmpersandEquals { line, .. } => *line,
            Token::Pipe { line, .. } => *line,
            Token::DoublePipe { line, .. } => *line,
            Token::PipeEquals { line, .. } => *line,
            Token::Caret { line, .. } => *line,
            Token::CaretEquals { line, .. } => *line,
            Token::Tilde { line, .. } => *line,
            Token::TildeEquals { line, .. } => *line,
            Token::Percent { line, .. } => *line,
            Token::PercentEquals { line, .. } => *line,

            Token::Identifier { line, .. } => *line,
            Token::String { line, .. } => *line,
            Token::Number { line, .. } => *line,
            // Token::Bool                   { line, .. } => *line,

            // Token::True                   { line, .. } => *line,
            // Token::False                  { line, .. } => *line,
            Token::If { line, .. } => *line,
            Token::Else { line, .. } => *line,
            Token::For { line, .. } => *line,
            Token::While { line, .. } => *line,
            Token::Loop { line, .. } => *line,
            Token::Return { line, .. } => *line,
            // Token::Null { line, .. } => *line,
            Token::Selff { line, .. } => *line,
            Token::Super { line, .. } => *line,
            Token::Use { line, .. } => *line,
            Token::Struct { line, .. } => *line,
            Token::Impl { line, .. } => *line,
            Token::As { line, .. } => *line,

            Token::EndOfFile { line, .. } => *line,

            Token::Invalid { .. } => 0,
        }
    }

    pub fn column(&self) -> u32 {
        match self {
            Token::LeftParenthesis { column, .. } => *column,
            Token::RightParenthesis { column, .. } => *column,
            Token::LeftAngledBracket { column, .. } => *column,
            Token::RightAngledBracket { column, .. } => *column,
            Token::LeftSquareBracket { column, .. } => *column,
            Token::RightSquareBracket { column, .. } => *column,
            Token::LeftSquigglyBracket { column, .. } => *column,
            Token::RightSquigglyBracket { column, .. } => *column,
            Token::Hashtag { column, .. } => *column,
            Token::Comma { column, .. } => *column,
            Token::Period { column, .. } => *column,
            Token::Colon { column, .. } => *column,
            Token::Semicolon { column, .. } => *column,
            Token::QuotationMark { column, .. } => *column,
            Token::Apostrophe { column, .. } => *column,

            Token::Plus { column, .. } => *column,
            Token::Minus { column, .. } => *column,
            Token::Asterisk { column, .. } => *column,
            Token::ForwardSlash { column, .. } => *column,
            Token::ExclamationPoint { column, .. } => *column,
            Token::LessThan { column, .. } => *column,
            Token::GreaterThan { column, .. } => *column,
            Token::Equals { column, .. } => *column,
            Token::DoubleEquals { column, .. } => *column,
            Token::PlusEquals { column, .. } => *column,
            Token::MinusEquals { column, .. } => *column,
            Token::AsteriskEquals { column, .. } => *column,
            Token::ForwardSlashEquals { column, .. } => *column,
            Token::ExclamationPointEquals { column, .. } => *column,
            Token::LessThanEquals { column, .. } => *column,
            Token::GreaterThanEquals { column, .. } => *column,
            Token::RightArrow { column, .. } => *column,
            Token::Ampersand { column, .. } => *column,
            Token::DoubleAmpersand { column, .. } => *column,
            Token::AmpersandEquals { column, .. } => *column,
            Token::Pipe { column, .. } => *column,
            Token::DoublePipe { column, .. } => *column,
            Token::PipeEquals { column, .. } => *column,
            Token::Caret { column, .. } => *column,
            Token::CaretEquals { column, .. } => *column,
            Token::Tilde { column, .. } => *column,
            Token::TildeEquals { column, .. } => *column,
            Token::Percent { column, .. } => *column,
            Token::PercentEquals { column, .. } => *column,

            Token::Identifier { column, .. } => *column,
            Token::String { column, .. } => *column,
            Token::Number { column, .. } => *column,
            // Token::Bool                   { column, .. } => *column,

            // Token::True                   { column, .. } => *column,
            // Token::False                  { column, .. } => *column,
            Token::If { column, .. } => *column,
            Token::Else { column, .. } => *column,
            Token::For { column, .. } => *column,
            Token::While { column, .. } => *column,
            Token::Loop { column, .. } => *column,
            Token::Return { column, .. } => *column,
            // Token::Null { column, .. } => *column,
            Token::Selff { column, .. } => *column,
            Token::Super { column, .. } => *column,
            Token::Use { column, .. } => *column,
            Token::Struct { column, .. } => *column,
            Token::Impl { column, .. } => *column,
            Token::As { column, .. } => *column,

            Token::EndOfFile { column, .. } => *column,

            Token::Invalid { .. } => 0,
        }
    }

    pub fn lexeme(&self) -> String {
        match self {
            Token::LeftParenthesis { .. } => "(".into(),
            Token::RightParenthesis { .. } => ")".into(),
            Token::LeftAngledBracket { .. } => "<".into(),
            Token::RightAngledBracket { .. } => ">".into(),
            Token::LeftSquareBracket { .. } => "[".into(),
            Token::RightSquareBracket { .. } => "]".into(),
            Token::LeftSquigglyBracket { .. } => "{".into(),
            Token::RightSquigglyBracket { .. } => "}".into(),
            Token::Hashtag { .. } => "#".into(),
            Token::Comma { .. } => ",".into(),
            Token::Period { .. } => ".".into(),
            Token::Colon { .. } => ":".into(),
            Token::Semicolon { .. } => ";".into(),
            Token::QuotationMark { .. } => "\"".into(),
            Token::Apostrophe { .. } => "\'".into(),

            Token::Plus { .. } => "+".into(),
            Token::Minus { .. } => "-".into(),
            Token::Asterisk { .. } => "*".into(),
            Token::ForwardSlash { .. } => "/".into(),
            Token::ExclamationPoint { .. } => "!".into(),
            Token::LessThan { .. } => "<".into(),
            Token::GreaterThan { .. } => ">".into(),
            Token::Equals { .. } => "=".into(),
            Token::DoubleEquals { .. } => "==".into(),
            Token::PlusEquals { .. } => "+=".into(),
            Token::MinusEquals { .. } => "-=".into(),
            Token::AsteriskEquals { .. } => "*=".into(),
            Token::ForwardSlashEquals { .. } => "/=".into(),
            Token::ExclamationPointEquals { .. } => "!=".into(),
            Token::LessThanEquals { .. } => "<=".into(),
            Token::GreaterThanEquals { .. } => ">=".into(),
            Token::RightArrow { .. } => "->".into(),
            Token::Ampersand { .. } => "&".into(),
            Token::DoubleAmpersand { .. } => "&&".into(),
            Token::AmpersandEquals { .. } => "&=".into(),
            Token::Pipe { .. } => "|".into(),
            Token::DoublePipe { .. } => "||".into(),
            Token::PipeEquals { .. } => "|=".into(),
            Token::Caret { .. } => "^".into(),
            Token::CaretEquals { .. } => "^=".into(),
            Token::Tilde { .. } => "~".into(),
            Token::TildeEquals { .. } => "~=".into(),
            Token::Percent { .. } => "%".into(),
            Token::PercentEquals { .. } => "%=".into(),

            Token::Identifier { lexeme, .. } => lexeme.clone(),
            Token::String { lexeme, .. } => lexeme.clone(),
            Token::Number { lexeme, .. } => lexeme.to_string(),
            // Token::Bool                   { lexeme, .. } => lexeme.to_string(),

            // Token::True                   { .. } => "true".into(),
            // Token::False                  { .. } => "false".into(),
            Token::If { .. } => "if".into(),
            Token::Else { .. } => "else".into(),
            Token::For { .. } => "for".into(),
            Token::While { .. } => "while".into(),
            Token::Loop { .. } => "loop".into(),
            Token::Return { .. } => "return".into(),
            // Token::Null { .. } => "null".into(),   
            Token::Selff { .. } => "self".into(),
            Token::Super { .. } => "super".into(),
            Token::Use { .. } => "use".into(),
            Token::Struct { .. } => "struct".into(),
            Token::Impl { .. } => "impl".into(),
            Token::As { .. } => "as".into(),

            Token::EndOfFile { .. } => "[EOF]".into(),

            Token::Invalid { .. } => "[INV]".into(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme())
    }
}

impl TokenDiscriminants {
    pub fn to_defaulted_token(&self, line: u32, column: u32) -> Token {
        match self {
            TokenDiscriminants::LeftParenthesis => Token::LeftParenthesis { line, column },
            TokenDiscriminants::RightParenthesis => Token::RightParenthesis { line, column },
            TokenDiscriminants::LeftAngledBracket => Token::LeftAngledBracket { line, column },
            TokenDiscriminants::RightAngledBracket => Token::RightAngledBracket { line, column },
            TokenDiscriminants::LeftSquareBracket => Token::LeftSquareBracket { line, column },
            TokenDiscriminants::RightSquareBracket => Token::RightSquareBracket { line, column },
            TokenDiscriminants::LeftSquigglyBracket => Token::LeftSquigglyBracket { line, column },
            TokenDiscriminants::RightSquigglyBracket => {
                Token::RightSquigglyBracket { line, column }
            }
            TokenDiscriminants::Hashtag => Token::Hashtag { line, column },
            TokenDiscriminants::Comma => Token::Comma { line, column },
            TokenDiscriminants::Period => Token::Period { line, column },
            TokenDiscriminants::Colon => Token::Colon { line, column },
            TokenDiscriminants::Semicolon => Token::Semicolon { line, column },
            TokenDiscriminants::QuotationMark => Token::QuotationMark { line, column },
            TokenDiscriminants::Apostrophe => Token::Apostrophe { line, column },

            TokenDiscriminants::Plus => Token::Plus { line, column },
            TokenDiscriminants::Minus => Token::Minus { line, column },
            TokenDiscriminants::Asterisk => Token::Asterisk { line, column },
            TokenDiscriminants::ForwardSlash => Token::ForwardSlash { line, column },
            TokenDiscriminants::ExclamationPoint => Token::ExclamationPoint { line, column },
            TokenDiscriminants::LessThan => Token::LessThan { line, column },
            TokenDiscriminants::GreaterThan => Token::GreaterThan { line, column },
            TokenDiscriminants::Equals => Token::Equals { line, column },
            TokenDiscriminants::DoubleEquals => Token::DoubleEquals { line, column },
            TokenDiscriminants::PlusEquals => Token::PlusEquals { line, column },
            TokenDiscriminants::MinusEquals => Token::MinusEquals { line, column },
            TokenDiscriminants::AsteriskEquals => Token::AsteriskEquals { line, column },
            TokenDiscriminants::ForwardSlashEquals => Token::ForwardSlashEquals { line, column },
            TokenDiscriminants::ExclamationPointEquals => {
                Token::ExclamationPointEquals { line, column }
            }
            TokenDiscriminants::LessThanEquals => Token::LessThanEquals { line, column },
            TokenDiscriminants::GreaterThanEquals => Token::GreaterThanEquals { line, column },
            TokenDiscriminants::Ampersand => Token::Ampersand { line, column },
            TokenDiscriminants::RightArrow => Token::RightArrow { line, column },
            TokenDiscriminants::DoubleAmpersand => Token::DoubleAmpersand { line, column },
            TokenDiscriminants::AmpersandEquals => Token::AmpersandEquals { line, column },
            TokenDiscriminants::Pipe => Token::Pipe { line, column },
            TokenDiscriminants::DoublePipe => Token::DoublePipe { line, column },
            TokenDiscriminants::PipeEquals => Token::PipeEquals { line, column },
            TokenDiscriminants::Caret => Token::Caret { line, column },
            TokenDiscriminants::CaretEquals => Token::CaretEquals { line, column },
            TokenDiscriminants::Tilde => Token::Tilde { line, column },
            TokenDiscriminants::TildeEquals => Token::TildeEquals { line, column },
            TokenDiscriminants::Percent => Token::Percent { line, column },
            TokenDiscriminants::PercentEquals => Token::PercentEquals { line, column },

            TokenDiscriminants::Identifier => Token::Identifier {
                line,
                column,
                lexeme: String::default(),
            },
            TokenDiscriminants::String => Token::String {
                line,
                column,
                lexeme: String::default(),
            },
            TokenDiscriminants::Number => Token::Number {
                line,
                column,
                lexeme: f64::default(),
            },
            // ToDiscriminantsken::Bool                   { lexeme, .. } => lexeme.to_string(),

            // ToDiscriminantsken::True                   { .. } => "true".into(),
            // ToDiscriminantsken::False                  { .. } => "false".into(),
            TokenDiscriminants::If      => Token::If { line, column },
            TokenDiscriminants::Else    => Token::Else { line, column },
            TokenDiscriminants::For     => Token::For { line, column },
            TokenDiscriminants::While   => Token::While { line, column },
            TokenDiscriminants::Loop    => Token::Loop { line, column },
            TokenDiscriminants::Return  => Token::Return { line, column },
            TokenDiscriminants::Selff   => Token::Selff { line, column },
            TokenDiscriminants::Super   => Token::Super { line, column },
            TokenDiscriminants::Use     => Token::Use { line, column },
            TokenDiscriminants::Struct  => Token::Struct { line, column },
            TokenDiscriminants::Impl    => Token::Impl { line, column },
            TokenDiscriminants::As      => Token::As { line, column },

            TokenDiscriminants::EndOfFile => Token::EndOfFile { line, column },

            TokenDiscriminants::Invalid => Token::Invalid {
                error: InterpreterError::UnknownToken {
                    line,
                    column,
                    location: "".into(),
                    token: "".into(),
                },
            },
        }
    }
}
