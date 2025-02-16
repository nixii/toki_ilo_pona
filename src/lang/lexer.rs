
use super::token::Token;

pub struct Lexer {
    text: String
}

// Implement the lexer
impl Lexer {

    // Create a new lexer
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    // Run the lexer!
    pub fn run_lexer(&self) -> Vec<Token> {
        
        // Create the vec of tokens
        let mut tokens: Vec<Token> = Vec::new();

        // Iterate through the characters
        for c in self.text.chars() {
            tokens.push(Token::LeftParen);
        }

        // Return the vec of tokens
        tokens
    }    
}