
pub mod token;
pub mod lexer;
pub mod error;
pub mod location;

use lexer::Lexer;

fn main() {
    let lexer = Lexer::new("1 + 2220.564356 4.3".to_owned());
    let tokens = lexer.tokenize();

    match tokens {
        Err(e) => println!("{}", e.get()),
        Ok(t) => println!("{:?}", t)
    }
}