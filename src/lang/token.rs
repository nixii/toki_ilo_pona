
// The different token types. Keywords are also used in base types.
#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    
    Add,
    Sub,
    Mul,
    Div,

    Number(f64)
}