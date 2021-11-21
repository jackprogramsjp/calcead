#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
}
