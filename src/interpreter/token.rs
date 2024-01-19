use std::fmt::{Display, Formatter};

use crate::error::InterpreterError;

#[derive(Debug, Clone, PartialEq)]
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
            Keyword::IF => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::If,
            }),
            Keyword::ELSE => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Else,
            }),
            Keyword::FOR => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::For,
            }),
            Keyword::WHILE => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::While,
            }),
            Keyword::LOOP => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Loop,
            }),
            Keyword::RETURN => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Return,
            }),
            Keyword::SELF => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::_Self,
            }),
            Keyword::SUPER => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Super,
            }),
            Keyword::IMPORT => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Import,
            }),
            Keyword::TYPE => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Type,
            }),
            Keyword::IMPL => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Impl,
            }),
            Keyword::AS => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::As,
            }),
            Keyword::VOID => Some(Token::Keyword {
                line,
                column,
                keyword: Keyword::Void,
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
    _Self,
    _CapitalSelf,
    Super,
    Export,
    Import,
    Public,
    Type,
    Impl,
    As,
    Void,
}

impl Keyword {
    pub const IF: &'static str = "if";
    pub const ELSE: &'static str = "else";
    pub const FOR: &'static str = "for";
    pub const WHILE: &'static str = "while";
    pub const LOOP: &'static str = "loop";
    pub const RETURN: &'static str = "return";
    pub const SELF: &'static str = "self";
    pub const CAPITAL_SELF: &'static str = "Self";
    pub const SUPER: &'static str = "super";
    pub const EXPORT: &'static str = "export";
    pub const IMPORT: &'static str = "import";
    pub const PUBLIC: &'static str = "import";
    pub const TYPE: &'static str = "type";
    pub const IMPL: &'static str = "impl";
    pub const AS: &'static str = "as";
    pub const VOID: &'static str = "void";

    pub fn lexeme(&self) -> String {
        match self {
            Keyword::If           => Keyword::IF.into(),
            Keyword::Else         => Keyword::ELSE.into(),
            Keyword::For          => Keyword::FOR.into(),
            Keyword::While        => Keyword::WHILE.into(),
            Keyword::Loop         => Keyword::LOOP.into(),
            Keyword::Return       => Keyword::RETURN.into(),
            Keyword::_Self        => Keyword::SELF.into(),
            Keyword::_CapitalSelf => Keyword::CAPITAL_SELF.into(),
            Keyword::Super        => Keyword::SUPER.into(),
            Keyword::Export       => Keyword::EXPORT.into(),
            Keyword::Import       => Keyword::IMPORT.into(),
            Keyword::Public       => Keyword::PUBLIC.into(),
            Keyword::Type         => Keyword::TYPE.into(),
            Keyword::Impl         => Keyword::IMPL.into(),
            Keyword::As           => Keyword::AS.into(),
            Keyword::Void         => Keyword::VOID.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
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
        }
    }
}
