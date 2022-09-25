use crate::parser::TokenType;

use super::{tree::Program, Parser, ParserError};

impl<'a> Parser<'a> {
    pub fn parse(&self) -> Result<Program, ParserError> {
        let out_stmts = Vec::new();

        loop {
            if let TokenType::EOF = self.lexer.peek().t {
                return Ok(out_stmts);
            }
        }
    }
}
