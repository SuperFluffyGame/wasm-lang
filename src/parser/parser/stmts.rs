use std::io::BufRead;

use super::Parser;
use crate::{
    match_tok,
    parser::{Stmt, StmtType, Token, TokenType},
    parser_error,
};

impl<'a> Parser<'a> {
    fn let_stmt(&mut self) -> Stmt {
        // let let_tok = self.lexer.next();
        // let ident_tok = self.lexer.next();
        // if let TokenType::Ident(i) = ident_tok.t {
        //     let eq_tok = self.lexer.next();
        //     if let TokenType::Equal = eq_tok.t {
        //         let expr = self.expr();
        //         let semi_tok = self.lexer.next();
        //         if let TokenType::Semi = semi_tok.t {
        //             Stmt::new(StmtType::Let(i, expr), let_tok.line, let_tok.line)
        //         } else {
        //             parser_error!(S; self, semi_tok, vec![TokenType::Semi])
        //         }
        //     } else {
        //         parser_error!(S; self, eq_tok, vec![TokenType::Equal])
        //     }
        // } else {
        //     parser_error!(S; self, ident_tok, vec![TokenType::Ident(String::new())])
        // }
        // todo!()
        match_tok!(S; self, Ident(i) => match_tok!(
            S; self, Equal => {
                let expr = self.expr();
                match_tok!(S; self, Semi => Stmt::new(StmtType::Let(i, expr), let_tok.line, let_tok.line))
            }
        ))
    }
    fn stmt_macro_goal(&mut self) -> Stmt {
        match_tok![S; self, Semi => Stmt::new(StmtType::Error, 0, 0)]
    }

    const STMT_EXPECTS: &'static [TokenType] = &[];
    pub(super) fn stmt(&mut self) -> Stmt {
        match self.lexer.peek().t {
            TokenType::KwLet => self.let_stmt(),
            _ => {
                let e = self.expr();
                let line = e.line;
                let col = e.col;
                Stmt::new(StmtType::Expr(e), line, col)
            }
        }
    }
}
