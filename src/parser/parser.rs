mod operators;
mod stmts;
mod parse;
pub mod tree;
use super::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum ParserErrorType {
    ExpectedButGot(Vec<TokenType>, TokenType),
}
#[derive(Debug)]
pub struct ParserError {
    t: ParserErrorType,
    line: i32,
    col: i32,
}
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub errors: Vec<ParserError>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, e: ParserError) {
        self.errors.push(e);
    }
}
