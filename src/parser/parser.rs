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
        Expr::new(ExprType::Error, $tok.line, $tok.col)
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
    (S; $self:ident, $($tok:ident $pat:pat => $then:expr),*) => {
        {
            let tok = $self.lexer.next();
            match tok.t {
                $(
                crate::parser::lexer::tokens::TokenType::$tok => $then,
                )*
                _ => parser_error!(S; $self, tok, vec![
                    $(crate::parser::lexer::tokens::TokenType::$tok,)*
                ])
            }
        }
    };
}
