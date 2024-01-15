use std::fmt::{Display, Formatter};

use crate::error::InterpreterError;

#[derive(Debug, Clone)]
pub enum Token {
    Symbol {
        line: u32,
        column: u32,
        symbol: Symbol,
    },
    Keyword {
        line: u32,
        column: u32,
        keyword: Keyword,
    },
    Literal {
        line: u32,
        column: u32,
        literal: Literal,
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
    pub fn line(&self) -> u32 {
        match self {
            Token::Symbol { line, .. } => *line,
            Token::Keyword { line, .. } => *line,
            Token::Literal { line, .. } => *line,
            Token::EndOfFile { line, .. } => *line,
            Token::Invalid { .. } => 0,
        }
    }

    pub fn column(&self) -> u32 {
        match self {
            Token::Symbol { column, .. } => *column,
            Token::Keyword { column, .. } => *column,
            Token::Literal { column, .. } => *column,
            Token::EndOfFile { column, .. } => *column,
            Token::Invalid { .. } => 0,
        }
    }

    pub fn lexeme(&self) -> String {
        match self {
            Token::Symbol { symbol, .. } => symbol.lexeme(),
            Token::Keyword { keyword, .. } => keyword.lexeme(),
            Token::Literal { literal, .. } => literal.lexeme(),
            Token::EndOfFile { .. } => "[EOF]".into(),
            Token::Invalid { .. } => "[INV]".into(),
        }
    }

    pub fn reserved_word(value: &str, line: u32, column: u32) -> Option<Self> {
        match value {
            // Keywords
            "if" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::If,
            }),
            "else" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Else,
            }),
            "for" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::For,
            }),
            "while" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::While,
            }),
            "loop" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Loop,
            }),
            "return" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Return,
            }),
            "self" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Selff,
            }),
            "super" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Super,
            }),
            "include" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Include,
            }),
            "struct" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Struct,
            }),
            "impl" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Impl,
            }),
            "as" => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::As,
            }),
            // Literals
            "void" => Some(Token::Literal {
                line,
                column,
                literal: Literal::Void,
            }),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    // Single-character
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    LeftSquigglyBracket,
    RightSquigglyBracket,
    Hashtag,
    Comma,
    Period,
    Colon,
    Semicolon,
    QuotationMark,
    Apostrophe,

    // One or two characters
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    ExclamationPoint,
    LeftAngledBracket,
    RightAngledBracket,
    Equals,
    DoubleEquals,
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    ForwardSlashEquals,
    ExclamationPointEquals,
    LeftAngledBracketEquals,
    RightAngledBracketEquals,
    RightArrow,
    Ampersand,
    DoubleAmpersand,
    AmpersandEquals,
    Pipe,
    DoublePipe,
    PipeEquals,
    Caret,
    CaretEquals,
    Tilde,
    TildeEquals,
    Percent,
    PercentEquals,
}

impl Symbol {
    pub fn lexeme(&self) -> String {
        match self {
            Symbol::LeftParenthesis { .. } => "(".into(),
            Symbol::RightParenthesis { .. } => ")".into(),
            Symbol::LeftAngledBracket { .. } => "<".into(),
            Symbol::RightAngledBracket { .. } => ">".into(),
            Symbol::LeftSquareBracket { .. } => "[".into(),
            Symbol::RightSquareBracket { .. } => "]".into(),
            Symbol::LeftSquigglyBracket { .. } => "{".into(),
            Symbol::RightSquigglyBracket { .. } => "}".into(),
            Symbol::Hashtag { .. } => "#".into(),
            Symbol::Comma { .. } => ",".into(),
            Symbol::Period { .. } => ".".into(),
            Symbol::Colon { .. } => ":".into(),
            Symbol::Semicolon { .. } => ";".into(),
            Symbol::QuotationMark { .. } => "\"".into(),
            Symbol::Apostrophe { .. } => "\'".into(),

            Symbol::Plus { .. } => "+".into(),
            Symbol::Minus { .. } => "-".into(),
            Symbol::Asterisk { .. } => "*".into(),
            Symbol::ForwardSlash { .. } => "/".into(),
            Symbol::ExclamationPoint { .. } => "!".into(),
            Symbol::Equals { .. } => "=".into(),
            Symbol::DoubleEquals { .. } => "==".into(),
            Symbol::PlusEquals { .. } => "+=".into(),
            Symbol::MinusEquals { .. } => "-=".into(),
            Symbol::AsteriskEquals { .. } => "*=".into(),
            Symbol::ForwardSlashEquals { .. } => "/=".into(),
            Symbol::ExclamationPointEquals { .. } => "!=".into(),
            Symbol::LeftAngledBracketEquals { .. } => "<=".into(),
            Symbol::RightAngledBracketEquals { .. } => ">=".into(),
            Symbol::RightArrow { .. } => "->".into(),
            Symbol::Ampersand { .. } => "&".into(),
            Symbol::DoubleAmpersand { .. } => "&&".into(),
            Symbol::AmpersandEquals { .. } => "&=".into(),
            Symbol::Pipe { .. } => "|".into(),
            Symbol::DoublePipe { .. } => "||".into(),
            Symbol::PipeEquals { .. } => "|=".into(),
            Symbol::Caret { .. } => "^".into(),
            Symbol::CaretEquals { .. } => "^=".into(),
            Symbol::Tilde { .. } => "~".into(),
            Symbol::TildeEquals { .. } => "~=".into(),
            Symbol::Percent { .. } => "%".into(),
            Symbol::PercentEquals { .. } => "%=".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    If,
    Else,
    For,
    While,
    Loop,
    Return,
    Selff,
    Super,
    Include,
    Struct,
    Impl,
    As,
}

impl Keyword {
    pub fn lexeme(&self) -> String {
        match self {
            Keyword::If { .. } => "if".into(),
            Keyword::Else { .. } => "else".into(),
            Keyword::For { .. } => "for".into(),
            Keyword::While { .. } => "while".into(),
            Keyword::Loop { .. } => "loop".into(),
            Keyword::Return { .. } => "return".into(),
            // Token::Null { .. } => "null".into(),
            Keyword::Selff { .. } => "self".into(),
            Keyword::Super { .. } => "super".into(),
            Keyword::Include { .. } => "use".into(),
            Keyword::Struct { .. } => "struct".into(),
            Keyword::Impl { .. } => "impl".into(),
            Keyword::As { .. } => "as".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Void,
    // Literals
    Identifier { lexeme: String },
    String { lexeme: String },
    Number { lexeme: f64 },
}

impl Literal {
    pub fn lexeme(&self) -> String {
        match self {
            Literal::Identifier { lexeme, .. } => lexeme.clone(),
            Literal::String { lexeme, .. } => lexeme.clone(),
            Literal::Number { lexeme, .. } => lexeme.to_string(),
            Literal::Void { .. } => "void".into(),
        }
    }
}
