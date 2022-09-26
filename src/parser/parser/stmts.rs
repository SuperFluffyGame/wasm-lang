use super::Parser;
use crate::parser::{Stmt, StmtType, TokenType};

impl<'a> Parser<'a> {
    // fn let_stmt(&mut self) -> Stmt {
    //     if let TokenType::KwLet = self.lexer.peek().t {
    //         self.lexer.next();
    //         if let TokenType::Ident(_) = self.lexer.peek().t {}
    //         if let TokenType::Equal = self.lexer.peek().t {
    //             self.lexer.next();
    //         }
    //     }
    // }

    pub(crate) fn stmt(&mut self) -> Stmt {
        let e = self.expr();
        let line = e.line;
        let col = e.col;
        Stmt::new(StmtType::Expr(e), line, col)
    }
}
