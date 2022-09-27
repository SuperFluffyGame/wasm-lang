#[allow(dead_code)]
#[derive(Debug, Clone)]
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

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub line: i32,
    pub col: i32,
}
impl Token {
    pub const fn new(t: TokenType, line: i32, col: i32) -> Self {
        Self { t, line, col }
    }
}
