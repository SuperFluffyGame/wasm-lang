use super::Parser;
use crate::{
    match_tok_single,
    parser::{parser::expects::Expects, Stmt, StmtType, TokenType},
};

impl<'a> Parser<'a> {
    fn let_stmt(&mut self, expects: Vec<Expects>) -> Stmt {
        match_tok_single!(S; self; ltok; Expects::Let; KwLet =>
        match_tok_single!(S; self; tok; Expects::Ident; Ident(i) =>
        match_tok_single!(S; self; tok; Expects::Equal; Equal =>{
            let e = self.expr(expects);
            match_tok_single!(S; self; tok; Expects::Semi; Semi => Stmt::new(StmtType::Let(i, e), ltok.line, ltok.col))
        })))
    }

    fn fn_decl_stmt(&mut self, expects: Vec<Expects>) -> Stmt {
        // match_tok_single!(S; self; kw_fn_tok; Expects::Fn)
        todo!()
    }

    // const STMT_EXPECTS: &'static [TokenType] = &[Expects::LetStmt];
    pub(super) fn stmt(&mut self) -> Stmt {
        match self.lexer.peek().t {
            TokenType::KwLet => self.let_stmt(vec![Expects::Stmt]),
            TokenType::KwFn => self.fn_decl_stmt(vec![Expects::Stmt]),
            _ => {
                let e = self.expr(vec![Expects::Stmt]);
                let line = e.line;
                let col = e.col;
                match_tok_single!(S; self; tok; Expects::Semi; Semi => {
                    Stmt::new(StmtType::Expr(e), line, col)
                })
            }
        }
    }
}
