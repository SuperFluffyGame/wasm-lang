#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Ident(String),
    Comment(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    Semi,
    KwLet,
    KwFn,

    LParen,
    RParen,
    LBrace,
    RBrace,
}
#[derive(Debug)]
pub struct Token {
    t: TokenType,
    line: i32,
    col: i32,
}
impl Token {
    pub fn new(t: TokenType, line: i32, col: i32) -> Self {
        Self { t, line, col }
    }
}
