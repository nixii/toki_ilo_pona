
use std::iter::{self, from_fn};

use crate::error::{illegal_char_error::IllegalCharError, Error};
use crate::location::Location;
use crate::token::{Token, TokenInfo};


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
        let mut iter = self.text.chars().peekable();

        let mut loc = Location::new(0, 0);

        while let Some(ch) = iter.next() {
            let mut ignore = false;

            match ch {
                '+' => tokens.push(Token::new(TokenInfo::Add, loc.clone())),
                '-' => tokens.push(Token::new(TokenInfo::Subtract, loc.clone())),
                '*' => tokens.push(Token::new(TokenInfo::Multiply, loc.clone())),
                '/' => tokens.push(Token::new(TokenInfo::Divide, loc.clone())),
                '1'..='9' => {
                    let mut dot = false;
                    let whole_number: f64 = iter::once(ch)
                        .chain(from_fn(|| iter.by_ref().next_if(|s| {
                            if *s == '.' {
                                dot = !dot;
                            }
                            loc.next_col();
                            s.is_ascii_digit() || (*s == '.' && dot)
                        })))
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    
                    tokens.push(Token::new(TokenInfo::Number(whole_number), loc.clone()));
                },
                '\n' => {
                    loc.next_row();
                    ignore = true;
                },
                ch if ch.is_whitespace() => continue,
                _ => return Err(Box::new(IllegalCharError::new(loc, ch.to_string())))
            }

            if !ignore {loc.next_col(); }
        }

        Ok(tokens)
    }
}