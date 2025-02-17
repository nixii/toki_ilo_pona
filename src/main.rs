
pub mod token;
pub mod lexer;
pub mod error;
pub mod location;

use lexer::Lexer;

fn main() {
    let lexer = Lexer::new("+-+a".to_owned());
    let tokens = lexer.tokenize();

    match tokens {
        Err(e) => println!("{}", e.get()),
        Ok(t) => println!("{:?}", t)
    }
}