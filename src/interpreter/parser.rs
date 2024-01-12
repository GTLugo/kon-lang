// use std::collections::VecDeque;

// use tracing::*;

// use super::{lexer::Lexer, token::Token};

// pub struct Parser {
//     tokens: VecDeque<Token>,
//     // current_index: usize,
// }

// impl Parser {
//     pub fn new(source: String) -> Self {
//         let lexer = Lexer::new(&source);
//         let tokens: VecDeque<_> = lexer.collect();

//         for token in &tokens {
//             debug!("{token:?}");
//         }

//         Self {
//             tokens,
//             // current_index: 0,
//         }
//     }

//     pub fn parse(&mut self) {}
// }
