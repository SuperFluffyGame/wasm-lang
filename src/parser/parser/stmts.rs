use super::Parser;
use crate::{
    match_tok,
    parser::{Stmt, StmtType, TokenType},
};

impl<'a> Parser<'a> {
    fn let_stmt(&mut self) -> Stmt {
        match_tok!(S; self, kw_let_tok, [KwLet => {
        match_tok!(S; self, tok, [Ident(String::new()); Ident(i) =>
        match_tok!(S; self, tok, [Equal => {
            let expr = self.expr();
            match_tok!(S; self, tok, [Semi =>
                Stmt::new(StmtType::Let(i, expr), kw_let_tok.line, kw_let_tok.col)
            ])
        }])])}])
    }

    // const STMT_EXPECTS: &'static [TokenType] = &[];
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
