use std::collections::{HashMap, VecDeque};

use crate::error::error_handler::ErrorHandler;
use crate::error::LexerError;
use crate::interpreter::token::Token;

use super::character_provider::CharacterProvider;
use super::token::TokenDiscriminants;

pub struct Lexer {
    keywords: HashMap<String, TokenDiscriminants>,
    // tokens: VecDeque<Token>,
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

impl Lexer {
    pub fn new() -> Self {
        let keywords = HashMap::from([
            ("if".into(), TokenDiscriminants::If),
            ("else".into(), TokenDiscriminants::Else),
            ("for".into(), TokenDiscriminants::For),
            ("while".into(), TokenDiscriminants::While),
            ("loop".into(), TokenDiscriminants::Loop),
            ("return".into(), TokenDiscriminants::Return),
            ("self".into(), TokenDiscriminants::Selff),
            ("super".into(), TokenDiscriminants::Super),
            ("use".into(), TokenDiscriminants::Use),
            ("struct".into(), TokenDiscriminants::Struct),
            ("impl".into(), TokenDiscriminants::Impl),
            ("as".into(), TokenDiscriminants::As),
        ]);
        Self {
            keywords,
        }
    }

    pub fn scan(&mut self, location: &str, source: &str) -> Result<VecDeque<Token>, ErrorHandler> {
        let mut error_handler = ErrorHandler::new();
        let mut characters = CharacterProvider::new(source);

        let mut tokens = VecDeque::default();
        while let Some(token) = self.build_token(location, &mut characters, &mut error_handler) {
            tokens.push_back(token);
        }
        
        tokens.push_back(Token::EndOfFile {
            line: characters.current_line(),
            column: characters.current_column(),
        });

        if error_handler.had_error() {
            Err(error_handler)
        } else {
            Ok(tokens)
        }
    }

    fn build_token(&self, location: &str, characters: &mut CharacterProvider, error_handler: &mut ErrorHandler) -> Option<Token> {
        let next_character = characters.next()?;
        let mut lexeme = String::new();
        match next_character {
            letter if letter.is_ascii_alphabetic() || letter == '_' => {
                let start_of_lexeme = characters.current_column();

                // identifiers
                lexeme.push(letter);
                lexeme.push_str(&Self::read_lexeme_while(characters, |c| {
                    c.is_ascii_alphanumeric() || c == &'_'
                }));

                if let Some(token) = self.keywords.get(&lexeme) {
                    Some(token.to_defaulted_token(characters.current_line(), start_of_lexeme))
                } else {
                    Some(Token::Identifier {
                        line: characters.current_line(),
                        column: start_of_lexeme,
                        lexeme,
                    })
                }
            }
            digit if digit.is_ascii_digit() => {
                let start_of_lexeme = characters.current_column();

                // numbers
                lexeme.push(digit);
                lexeme.push_str(&Self::read_lexeme_while(characters, |c| c.is_ascii_digit()));

                if let Ok(lexeme) = lexeme.parse::<f64>() {
                    Some(Token::Number {
                        line: characters.current_line(),
                        column: start_of_lexeme,
                        lexeme,
                    })
                } else {
                    error_handler.lexing_error(LexerError::SyntaxError {
                        location: location.into(),
                        line: characters.current_line(),
                        column: characters.current_column(),
                        message: "Failed to parse number".into(),
                    })
                }
            }
            symbol if symbol.is_ascii_punctuation() => {
                // symbols
                match symbol {
                    ';' => Some(Token::Semicolon {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    ',' => Some(Token::Comma {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '.' => Some(Token::Period {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    ':' => Some(Token::Colon {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '!' => {
                        if Self::next_char_is(characters, '=') {
                            Some(Token::ExclamationPointEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::ExclamationPoint {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '=' => {
                        if Self::next_char_is(characters, '=') {
                            Some(Token::DoubleEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::Equals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '+' => {
                        if Self::next_char_is(characters, '=') {
                            Some(Token::PlusEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::Plus {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '-' => {
                        if Self::next_char_is(characters, '>') {
                            Some(Token::RightArrow {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else if Self::next_char_is(characters, '=') {
                            Some(Token::MinusEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::Minus {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '/' => {
                        if Self::next_char_is(characters, '=') {
                            Some(Token::ForwardSlashEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::ForwardSlash {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '*' => {
                        if Self::next_char_is(characters, '=') {
                            Some(Token::AsteriskEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::Asterisk {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '&' => {
                        if Self::next_char_is(characters, '=') {
                            Some(Token::AsteriskEquals {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        } else {
                            Some(Token::Asterisk {
                                line: characters.current_line(),
                                column: characters.current_column(),
                            })
                        }
                    }
                    '(' => Some(Token::LeftParenthesis {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    ')' => Some(Token::RightParenthesis {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '{' => Some(Token::LeftSquigglyBracket {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '}' => Some(Token::RightSquigglyBracket {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '<' => Some(Token::LeftAngledBracket {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '>' => Some(Token::RightAngledBracket {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '[' => Some(Token::LeftSquareBracket {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    ']' => Some(Token::RightSquareBracket {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '\'' => Some(Token::Apostrophe {
                        line: characters.current_line(),
                        column: characters.current_column(),
                    }),
                    '\"' => {
                        let start_of_lexeme = characters.current_column();

                        let is_end_of_string = Self::next_char_is(characters, '\"');
                        if let Some(_) = characters.peek()
                            && !is_end_of_string
                        {
                            while let Some(c) = characters.peek() {
                                if *c != '\"' {
                                    lexeme.push(characters.next_with_spaces().unwrap());
                                } else {
                                    characters.next().unwrap();
                                    return Some(Token::String {
                                        line: characters.current_line(),
                                        column: start_of_lexeme,
                                        lexeme,
                                    });
                                }
                            }
                        }

                        
                        error_handler.lexing_error(LexerError::UnterminatedString {
                            location: location.into(),
                            line: characters.current_line(),
                            column: characters.current_column(),
                        })
                    }
                    c => {
                        error_handler.lexing_error(LexerError::UnknownToken {
                            location: location.into(),
                            line: characters.current_line(),
                            column: characters.current_column(),
                            token: c.into(),
                        })
                    },
                }
            }
            c => {
                error_handler.lexing_error(LexerError::UnknownToken {
                    location: location.into(),
                    line: characters.current_line(),
                    column: characters.current_column(),
                    token: c.into(),
                })
            },
        }
    }

    fn next_char_is(characters: &mut CharacterProvider, character: char) -> bool {
        if let Some(c) = characters.peek()
            && c == &character
        {
            characters.next();
            true
        } else {
            false
        }
    }

    fn read_lexeme_while(characters: &mut CharacterProvider, condition: impl Fn(&char) -> bool) -> String {
        let mut lexeme = String::new();
        while let Some(c) = characters.peek() && condition(c) {
            lexeme.push(characters.next().unwrap());
        }
        lexeme
    }
}
