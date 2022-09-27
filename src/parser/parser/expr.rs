use crate::{match_tok, match_tok_peek, node};

use super::{super::ExprType, tree::Expr, Parser, TokenType};

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

macro_rules! expr_list {
    ($fn_name:ident, [$($expect:expr; $end_tok:pat),*]) => {
        fn $fn_name(&mut self) -> Vec<Expr>{
            let mut exprs = Vec::new();
            use TokenType::*;
            match self.lexer.peek().t {
                $(
                    $end_tok
                )|* => {self.lexer.next(); exprs},
                _ => {
                    exprs.push(self.expr());
                    let exprs = loop {

                        exprs.push(
                            match_tok_peek!(E; self, tok, [
                            Comma; Comma => {
                                self.lexer.next();
                                self.expr()
                            },
                            $($expect; $end_tok => {
                                self.lexer.next();
                                break exprs;
                            }),*
                            ,None; _ => break exprs
                            ])
                        );
                    };


                    exprs
                }
            }
        }
    };
     ($fn_name:ident, [$($end_tok:ident),*]) => {

            expr_list!($fn_name, [$($end_tok; $end_tok),*]);

     };
}

impl<'a> Parser<'a> {
    pub(super) fn expr(&mut self) -> Expr {
        self.add_expr()
    }
    binary_expr!(add_expr, mul_expr,   [Plus     => Add, Minus => Sub]);
    binary_expr!(mul_expr, unary_expr, [Asterisk => Mul, Slash => Div]);
    unary_expr!(primary_expr, [Minus => Neg]);
    // binary_expr!(exp_expr, primary_expr, [])

    expr_list!(fn_args, [RParen]);

    fn primary_expr(&mut self) -> Expr {
        match_tok!(E; self, tok, [
            IntLiteral(0); IntLiteral(i) => node!(E; tok, Int(i)),
            FloatLiteral(0.0); FloatLiteral(f) => node!(E; tok, Float(f)),
            LParen; LParen => {
                let e = self.expr();
                match_tok!(E; self, tok, [RParen => {
                    e
                }])
            },
            Ident(String::new()); Ident(s) => {
                let next_tok = self.lexer.peek();
                match next_tok.t {
                    LParen => {
                        let rparen_tok = self.lexer.next();
                        let e_list = self.fn_args();
                        node!(E; tok, FnCall(s, e_list))
                    }
                    _ => node!(E; tok, Ident(s))
                }
            }
        ])
    }
}