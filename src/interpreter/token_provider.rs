use std::{iter::Peekable, slice::Iter};

use super::token::Token;

#[derive(Debug, Clone)]
pub enum Next<T> {
    Token(T),
    EndOfFile { line: u32, column: u32 },
    EndOfStream { line: u32, column: u32 },
}

impl<T: Clone> Next<&T> {
    pub fn cloned(&self) -> Next<T> {
        match *self {
            Next::Token(token) => Next::Token(token.clone()),
            Next::EndOfFile { line, column } => Next::EndOfFile { line, column },
            Next::EndOfStream { line, column } => Next::EndOfStream { line, column },
        }
    }
}

pub struct TokenProvider<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    last_line: u32,
    last_column: u32,
}

impl<'a> TokenProvider<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            last_line: 0,
            last_column: 0,
        }
    }

    pub fn peek(&mut self) -> Next<&Token> {
        match self.tokens.peek() {
            Some(token) => match token {
                Token::EndOfFile { line, column } => {
                    self.last_line = *line;
                    self.last_column = *column;
                    Next::EndOfFile {
                        line: *line,
                        column: *column,
                    }
                }
                &t => Next::Token(t),
            },
            None => Next::EndOfStream {
                line: self.last_line,
                column: self.last_column,
            },
        }
    }

    pub fn next(&mut self) -> Next<&Token> {
        match self.tokens.next() {
            Some(token) => match token {
                &Token::EndOfFile { line, column } => Next::EndOfFile { line, column },
                t => Next::Token(t),
            },
            None => Next::EndOfStream {
                line: self.last_line,
                column: self.last_column,
            },
        }
    }
}

// impl<'a> Iterator for TokenProvider<'a> {
//     type Item = &'a Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.tokens.next() {
//             Some(token) => match token {
//                 &Token::EndOfFile { .. } => None,
//                 t => Some(t),
//             },
//             None => None,
//         }
//     }
// }
