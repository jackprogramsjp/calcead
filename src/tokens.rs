#[derive(Debug, Copy, Clone)]
pub enum Token {
    Number(f32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Lparen,
    Rparen,
}
