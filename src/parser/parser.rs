mod operators;
mod parse;
mod stmts;
pub mod tree;
use super::{Lexer, TokenType};

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

#[macro_export]
macro_rules! parser_error {
    (E; $self:ident, $tok:ident, $expects:expr) => {{
        $self.error(crate::parser::parser::ParserError {
            t: crate::parser::parser::ParserErrorType::ExpectedButGot($expects, $tok.t.clone()),
            line: $tok.line,
            col: $tok.col,
        });
        crate::parser::parser::tree::Expr::new(
            crate::parser::parser::tree::ExprType::Error,
            $tok.line,
            $tok.col,
        )
    }};
    (S; $self:ident, $tok:ident, $expects:expr) => {{
        $self.error(crate::parser::parser::ParserError {
            t: crate::parser::parser::ParserErrorType::ExpectedButGot($expects, $tok.t.clone()),
            line: $tok.line,
            col: $tok.col,
        });
        crate::parser::parser::tree::Stmt::new(
            crate::parser::parser::tree::StmtType::Error,
            $tok.line,
            $tok.col,
        )
    }};
}

#[macro_export]
macro_rules! match_tok {
    // unit variant with named tok var
    ($type:ident; $self:ident, $tok_var:ident, [$($tok:ident => $then:expr),*]) => {
        match_tok!($type; $self, $tok_var, [$($tok; $tok => $then),*])
    };

    // tuple variant with named token var
    ($type:ident; $self:ident, $tok_var:ident, [$($expect:expr; $pat:pat => $then:expr),*]) => {
        {
            use crate::parser::lexer::tokens::TokenType::*;
            let $tok_var = $self.lexer.next();
            match $tok_var.t {
                $(
                $pat => $then,
                )*
                _ => crate::parser_error!($type; $self, $tok_var, vec![
                    $($expect),*
                ])
            }
        }
    };
}

#[macro_export]
macro_rules! node {
    (S; $tok:ident, $type:expr) => {{
        use crate::parser::parser::tree::StmtType::*;
        crate::parser::parser::tree::Stmt::new($type, $tok.line, $tok.col)
    }};
    (E; $tok:ident, $type:expr) => {{
        use crate::parser::parser::tree::ExprType::*;
        crate::parser::parser::tree::Expr::new($type, $tok.line, $tok.col)
    }};
}
