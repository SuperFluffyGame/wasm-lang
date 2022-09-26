use crate::parser::ExprType;
use crate::parser::Token;

use super::tree::Expr;
use super::ParserError;
use super::ParserErrorType;

use super::Parser;
use super::TokenType;

macro_rules! binary_expr {
    // left associative
    ($ident: ident, $higher_prec_op: ident, [$($token: ident => $expr: ident),*]) => {
        fn $ident(&mut self) -> Expr {
            let mut a = self.$higher_prec_op();
            loop {
                $(
                    if let TokenType::$token = self.lexer.peek().t {
                        self.lexer.next();
                        let b = self.$higher_prec_op();
                        let line = a.line;
                        let col = a.col;
                        a = Expr::new(ExprType::$expr(Box::new(a), Box::new(b)), line, col);
                        continue;
                    }
                )*
                return a;
            }
        }
    };

    // right associative
    // (R; $ident:ident, [$($token:expr),+], $higher_prec_op:ident) => {};
}
macro_rules! unary_expr {
    ($higher_prec_op: ident, [$($token: ident => $expr: ident),*]) => {
        fn unary_expr(&mut self) -> Expr {
            $(
                if let TokenType::$token = self.lexer.peek().t {
                    let tok = self.lexer.next();
                    return Expr::new(ExprType::$expr(Box::new(self.$higher_prec_op())), tok.line, tok.col)
                }
            )*

            self.$higher_prec_op()
        }
    };
}

impl<'a> Parser<'a> {
    pub(crate) fn expr(&mut self) -> Expr {
        self.add_expr()
    }
    binary_expr!(add_expr, mul_expr,   [Plus     => Add, Minus => Sub]);
    binary_expr!(mul_expr, unary_expr, [Asterisk => Mul, Slash => Div]);
    unary_expr!(primary_expr, [Minus => Neg]);
    // binary_expr!(exp_expr, primary_expr, [])

    const PRIMARY_EXPR_EXPECTS: &'static [TokenType] = &[TokenType::IntLiteral(0)];
    fn primary_expr(&mut self) -> Expr {
        let tok = self.lexer.next();
        match tok.t {
            TokenType::IntLiteral(i) => Expr::new(ExprType::Int(i), tok.line, tok.col),
            TokenType::FloatLiteral(f) => Expr::new(ExprType::Float(f), tok.line, tok.col),
            TokenType::LParen => {
                let e = self.expr();
                if let TokenType::RParen = self.lexer.peek().t {
                    self.lexer.next();
                    e
                } else {
                    self.error(ParserError {
                        t: ParserErrorType::ExpectedButGot(vec![TokenType::RParen], tok.t.clone()),
                        line: tok.line,
                        col: tok.col,
                    });
                    Expr::new(ExprType::Error, tok.line, tok.col)
                }
            }
            _ => {
                self.error(ParserError {
                    t: ParserErrorType::ExpectedButGot(
                        Self::PRIMARY_EXPR_EXPECTS.to_vec(),
                        tok.t.clone(),
                    ),
                    line: tok.line,
                    col: tok.col,
                });
                Expr::new(ExprType::Error, tok.line, tok.col)
            }
        }
    }
}
