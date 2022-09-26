use crate::parser::TokenType;

use super::{
    tree::{Program, Stmt},
    Parser,
};

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Program {
        let mut out_stmts = Vec::new();

        loop {
            // println!("{:?}", self.lexer.peek().t);
            if let TokenType::EOF = self.lexer.peek().t {
                return out_stmts;
            }
            out_stmts.push(Stmt::Expr(self.expr()))
        }
    }
}
