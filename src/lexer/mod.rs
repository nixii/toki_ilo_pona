use std::fmt::Debug;

use crate::error::{illegal_char_error::IllegalCharError, Error};
use crate::location::Location;
use crate::token::{Token, TokenInfo, Keyword};


pub struct Lexer {
    text: String
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens: Vec<Token> = Vec::new();
        let iter = self.text.chars();

        let mut loc = Location::new(0, 0);

        let mut err: Option<Box<dyn Error>> = None;

        for ch in iter {
            match ch {
                '+' => tokens.push(Token::new(TokenInfo::Add, loc.clone())),
                '-' => tokens.push(Token::new(TokenInfo::Subtract, loc.clone())),
                '*' => tokens.push(Token::new(TokenInfo::Multiply, loc.clone())),
                '/' => tokens.push(Token::new(TokenInfo::Divide, loc.clone())),
                '\n' => {
                    loc.next_row();
                },
                ch if ch.is_whitespace() => {
                },
                _ => {
                    err = Some(Box::new(IllegalCharError::new(loc, ch.to_string())));
                    break;
                }
            }

            loc.next_col();
        }

        match err {
            Some(e) => Err(e),
            None => Ok(tokens)
        }
    }
}