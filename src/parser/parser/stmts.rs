use super::Parser;
use crate::{
    match_tok_single,
    parser::{parser::expects::Expects, Stmt, StmtType, TokenType},
};

impl<'a> Parser<'a> {
    fn let_stmt(&mut self) -> Stmt {
        match_tok_single!(S; self; ltok; Expects::Let; KwLet =>
        match_tok_single!(S; self; tok; Expects::Ident; Ident(i) =>
        match_tok_single!(S; self; tok; Expects::Equal; Equal =>{
            let e = self.expr();
            match_tok_single!(S; self; tok; Expects::Semi; Semi => Stmt::new(StmtType::Let(i, e), ltok.line, ltok.col))
        })))
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
