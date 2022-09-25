mod parse;
pub mod tree;

use super::{Lexer, Token};
pub enum ParserErrorType {
    ExpectedButGot(Token, Token),
}
pub struct ParserError {
    t: ParserErrorType,
    line: i32,
    col: i32,
}
pub struct Parser<'a> {
    index: i32,
    lexer: &'a Lexer<'a>,
    errors: Vec<ParserError>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer<'a>) -> Self {
        Self {
            index: 0,
            lexer,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, e: ParserError) {
        self.errors.push(e);
    }
}
