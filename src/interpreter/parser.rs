use crate::error::{error_handler::ErrorHandler, InterpreterError};

use super::{
    grammar::Expression,
    token::{Keyword, Literal, Symbol, Token},
    token_provider::{Next, TokenProvider},
};

macro_rules! match_next_token {
    ($tokens:expr, $pattern:pat) => {{
        if let Next::Token($pattern) = $tokens.peek() {
            true
        } else {
            false
        }
    }};
}

pub struct Parser<'a> {
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Parser<'a> {
    pub fn new(error_handler: &'a mut ErrorHandler) -> Self {
        Self { error_handler }
    }

    pub fn parse(&mut self, tokens: &[Token]) -> Expression {
        let mut tokens = TokenProvider::new(tokens);
        match self.expression(&mut tokens) {
            Ok(expression) => expression,
            Err(_error) => Expression::Invalid,
        }
    }

    fn expression(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        self.equality(tokens)
    }

    fn equality(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        let mut expression = self.comparison(tokens)?;

        while match_next_token!(
            tokens,
            Token::Symbol {
                symbol: Symbol::DoubleEquals | Symbol::ExclamationPointEquals,
                ..
            }
        ) {
            if let Next::Token(operator) = tokens.next().cloned() {
                let right_operand = Box::new(self.comparison(tokens)?);
                expression = Expression::Binary {
                    left_operand: Box::new(expression),
                    operator,
                    right_operand,
                };
            }
        }

        Ok(expression)
    }

    fn comparison(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        let mut expression = self.term(tokens)?;

        while match_next_token!(
            tokens,
            Token::Symbol {
                symbol: Symbol::LeftAngledBracket
                    | Symbol::RightAngledBracket
                    | Symbol::LeftAngledBracketEquals
                    | Symbol::RightAngledBracketEquals,
                ..
            }
        ) {
            if let Next::Token(operator) = tokens.next().cloned() {
                let right_operand = Box::new(self.term(tokens)?);
                expression = Expression::Binary {
                    left_operand: Box::new(expression),
                    operator,
                    right_operand,
                };
            }
        }

        Ok(expression)
    }

    fn term(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        let mut expression = self.factor(tokens)?;

        while match_next_token!(
            tokens,
            Token::Symbol {
                symbol: Symbol::Plus | Symbol::Minus,
                ..
            }
        ) {
            if let Next::Token(operator) = tokens.next().cloned() {
                let right_operand = Box::new(self.factor(tokens)?);
                expression = Expression::Binary {
                    left_operand: Box::new(expression),
                    operator,
                    right_operand,
                };
            }
        }

        Ok(expression)
    }

    fn factor(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        let mut expression = self.power(tokens)?;

        while match_next_token!(
            tokens,
            Token::Symbol {
                symbol: Symbol::Asterisk | Symbol::ForwardSlash,
                ..
            }
        ) {
            if let Next::Token(operator) = tokens.next().cloned() {
                let right_operand = Box::new(self.power(tokens)?);
                expression = Expression::Binary {
                    left_operand: Box::new(expression),
                    operator,
                    right_operand,
                };
            }
        }

        Ok(expression)
    }

    fn power(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        let mut expression = self.unary(tokens)?;

        while match_next_token!(
            tokens,
            Token::Symbol {
                symbol: Symbol::Caret,
                ..
            }
        ) {
            if let Next::Token(operator) = tokens.next().cloned() {
                let right_operand = Box::new(self.unary(tokens)?);
                expression = Expression::Binary {
                    left_operand: Box::new(expression),
                    operator,
                    right_operand,
                };
            }
        }

        Ok(expression)
    }

    fn unary(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        if match_next_token!(
            tokens,
            Token::Symbol {
                symbol: Symbol::ExclamationPoint | Symbol::Minus,
                ..
            }
        ) {
            if let Next::Token(operator) = tokens.next().cloned() {
                let operand = Box::new(self.unary(tokens)?);
                return Ok(Expression::Unary { operator, operand });
            }
        }

        self.primary(tokens)
    }

    fn primary(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
        match tokens.next() {
            Next::Token(token) => {
                match &token {
                    Token::Literal { literal, .. } => match literal {
                        Literal::String { .. } | Literal::Number { .. } => {
                            return Ok(Expression::Literal {
                                token: token.clone(),
                            })
                        }
                        _ => {}
                    },
                    Token::Symbol { symbol, .. } => match symbol {
                        Symbol::LeftParenthesis => {
                            let operand = Box::new(self.expression(tokens)?);
                            let _delimiter =
                                self.pair_delimiter(tokens, Symbol::RightParenthesis)?;
                            return Ok(Expression::Grouping { operand });
                        }
                        Symbol::LeftSquigglyBracket => {
                            let operand = Box::new(self.expression(tokens)?);
                            let _delimiter =
                                self.pair_delimiter(tokens, Symbol::RightSquigglyBracket)?;
                            return Ok(Expression::Grouping { operand });
                        }
                        _ => {}
                    },
                    _ => {}
                }

                self.error(InterpreterError::ParseError {
                    line: token.line(),
                    column: token.column(),
                    message: "expected expression".into(),
                })
            }
            Next::EndOfFile { line, column } => self.error(InterpreterError::ParseError {
                line,
                column,
                message: "expected expression".into(),
            }),
            Next::EndOfStream { line, column } => self.error(InterpreterError::ParseError {
                line,
                column,
                message: "expected expression".into(),
            }),
        }
    }

    fn pair_delimiter(
        &mut self,
        tokens: &mut TokenProvider,
        delimiter: Symbol,
    ) -> Result<Token, InterpreterError> {
        match self.check_delimiter(tokens, &delimiter) {
            Ok(matches) => match tokens.next().cloned() {
                Next::Token(token) => {
                    if matches {
                        Ok(token)
                    } else {
                        let delimiter = Token::Symbol {
                            line: token.line(),
                            column: token.column(),
                            symbol: delimiter,
                        };
                        self.error(InterpreterError::UnmatchedDelimiter {
                            line: delimiter.line(),
                            column: delimiter.column(),
                            delimiter: delimiter.lexeme(),
                        })
                    }
                }
                Next::EndOfFile { line, column } | Next::EndOfStream { line, column } => {
                    let delimiter = Token::Symbol {
                        line,
                        column,
                        symbol: delimiter,
                    };
                    self.error(InterpreterError::UnmatchedDelimiter {
                        line: delimiter.line(),
                        column: delimiter.column(),
                        delimiter: delimiter.lexeme(),
                    })
                }
            },
            Err((line, column)) => {
                let delimiter = Token::Symbol {
                    line,
                    column,
                    symbol: delimiter,
                };
                self.error(InterpreterError::UnmatchedDelimiter {
                    line: delimiter.line(),
                    column: delimiter.column(),
                    delimiter: delimiter.lexeme(),
                })
            }
        }
    }

    fn check_delimiter(
        &mut self,
        tokens: &mut TokenProvider,
        delimiter: &Symbol,
    ) -> Result<bool, (u32, u32)> {
        match tokens.peek() {
            Next::Token(Token::Symbol { symbol, .. }) => Ok(symbol == delimiter),
            Next::Token(token) => Err((token.line(), token.column())),
            Next::EndOfFile { line, column } | Next::EndOfStream { line, column } => {
                Err((line, column))
            }
        }
    }

    fn error<T>(&mut self, error: InterpreterError) -> Result<T, InterpreterError> {
        self.error_handler.push(error.clone());

        Err(error)
    }

    fn synchronize(&mut self, tokens: &mut TokenProvider) {
        let mut token = tokens.next();
        while let Next::Token(next_token) = token {
            if matches!(
                next_token,
                Token::Symbol {
                    symbol: Symbol::Semicolon,
                    ..
                }
            ) {
                return;
            }

            if matches!(
                next_token,
                Token::Keyword {
                    keyword: Keyword::If
                        | Keyword::For
                        | Keyword::While
                        | Keyword::Loop
                        | Keyword::Return
                        | Keyword::Struct
                        | Keyword::Impl,
                    ..
                }
            ) {
                return;
            }

            token = tokens.next();
        }
    }
}
