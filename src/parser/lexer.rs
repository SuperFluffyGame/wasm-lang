use super::{char_reader::CharReader, Token};

mod lex;
pub mod tokens;

#[derive(Debug)]
pub enum LexerErrorType {
    UnexpectedCharacter(char),
}
#[derive(Debug)]
pub struct LexerError {
    pub t: LexerErrorType,
    pub line: i32,
    pub col: i32,
}
pub struct Lexer {
    col: i32,
    line: i32,
    reader: CharReader,
    next: Option<Token>,
    pub errors: Vec<LexerError>,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut s = Self {
            col: 0,
            line: 0,
            reader: CharReader::new(input),
            next: None,
            errors: Vec::new(),
        };
        s._next();
        s
    }
    pub fn error(&mut self, t: LexerErrorType) {
        self.errors.push(LexerError {
            t,
            line: self.line,
            col: self.col,
        });
    }
}
