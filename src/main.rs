
// Make the lang module visible
pub mod lang;

// Get the lexer
use lang::lexer::Lexer;

// Run
fn main() {

    // Testing string
    let input_str = "1 + 2";

    // Lex the string to get tokens!
    let lexer = Lexer::new(String::from(input_str));
    let tokens = lexer.run_lexer();
    println!("{:?}", tokens)
}
