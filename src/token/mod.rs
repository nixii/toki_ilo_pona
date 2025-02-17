
// Use the location class
use crate::location::Location;

// Keywords
#[derive(Debug)]
pub enum Keyword {
    O,
    Nanpa,
    Nimi,
    Nasin,
    Linja,
}

// Different kinds of token types
#[derive(Debug)]
pub enum TokenInfo {
    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    Semicolon,

    Add,
    Subtract,
    Multiply,
    Divide,

    Number(f64),
    String(String),

    Identifier(String),
    Keyword(Keyword)
}

// Token information
#[derive(Debug)]
pub struct Token {
    info: TokenInfo,
    location: Location
}

// Implement functions
impl Token {
    pub fn new(info: TokenInfo, 
        location: Location) -> Self {

        Self {
            info,
            location
        }
    }
}