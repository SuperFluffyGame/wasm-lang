mod parse;
pub mod tree;

use super::Token;
pub enum ParserError {}
pub struct Parser<'a> {
    index: i32,
    tokens: &'a Vec<Token>,
    errors: Vec<ParserError>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, e: ParserError) {
        self.errors.push(e);
    }
}
