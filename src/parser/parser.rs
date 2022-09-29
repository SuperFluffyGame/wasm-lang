mod expects;
mod expr;
mod parse;
mod stmts;
pub mod tree;
use self::expects::Expects;

use super::{Lexer, TokenType};

#[derive(Debug)]
pub enum ParserErrorType {
    ExpectedButGot(Vec<Expects>, TokenType),
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
        $self.error($crate::parser::parser::ParserError {
            t: $crate::parser::parser::ParserErrorType::ExpectedButGot($expects, $tok.t),
            line: $tok.line,
            col: $tok.col,
        });
        $crate::parser::parser::tree::Expr::new(
            $crate::parser::parser::tree::ExprType::Error,
            $tok.line,
            $tok.col,
        )
    }};
    (S; $self:ident, $tok:ident, $expects:expr) => {{
        $self.error($crate::parser::parser::ParserError {
            t: $crate::parser::parser::ParserErrorType::ExpectedButGot($expects, $tok.t),
            line: $tok.line,
            col: $tok.col,
        });
        $crate::parser::parser::tree::Stmt::new(
            $crate::parser::parser::tree::StmtType::Error,
            $tok.line,
            $tok.col,
        )
    }};
}

#[macro_export]
macro_rules! match_tok_single {
    ($type:ident; $self:ident; $tok_var:ident; $expects:expr; $to_match:pat => $then:expr) => {{
        use $crate::parser::lexer::tokens::TokenType::*;

        let $tok_var = $self.lexer.next();
        if let $to_match = $tok_var.t.clone() {
            $then
        } else {
            $crate::parser_error!($type; $self, $tok_var, $expects)
        }
    }};
        ($type:ident; $self:ident; $tok_var:ident; $expects:expr; $to_match:pat => $then:expr) => {
            match_tok_single!($type; $self; $tok_var; [$expects]; $to_match => $then)
        }
}

#[macro_export]
macro_rules! node {
    (S; $tok:ident, $type:expr) => {{
        use $crate::parser::parser::tree::StmtType::*;
        $crate::parser::parser::tree::Stmt::new($type, $tok.line, $tok.col)
    }};
    (E; $tok:ident, $type:expr) => {{
        use $crate::parser::parser::tree::ExprType::*;
        $crate::parser::parser::tree::Expr::new($type, $tok.line, $tok.col)
    }};
}
