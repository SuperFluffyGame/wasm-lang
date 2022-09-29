use crate::{
    match_tok_single, node,
    parser::{parser::expects::Expects, Stmt},
    parser_error,
};

use super::{super::ExprType, tree::Expr, Parser, TokenType};

macro_rules! binary_expr {
    // left associative
    ($ident: ident, $higher_prec_op: ident, [$($token: ident => $expr: ident),*]) => {
        fn $ident(&mut self, expects: Vec<Expects>) -> Expr {
            let mut a = self.$higher_prec_op(expects.clone());
            loop {
                $(
                    if let TokenType::$token = self.lexer.peek().t {
                        self.lexer.next();
                        let b = self.$higher_prec_op(expects.clone());
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
        fn unary_expr(&mut self, expects: Vec<Expects>) -> Expr {
            $(
                if let TokenType::$token = self.lexer.peek().t {
                    let tok = self.lexer.next();
                    return Expr::new(ExprType::$expr(Box::new(self.$higher_prec_op(expects))), tok.line, tok.col)
                }
            )*

            self.$higher_prec_op(expects)
        }
    };
}
macro_rules! list {
    ($fn_name:ident; $end_tok_expect:expr; $end_tok:pat) => {
        fn $fn_name(&mut self, expects: Vec<Expects>) -> Vec<Expr>{
            let mut exprs = Vec::new();
            use TokenType::*;
            match self.lexer.peek().t {
                $end_tok => {self.lexer.next();},
                _ => {
                    exprs.push(self.expr(expects.clone()));
                    loop {
                        let tok = self.lexer.next();
                        match tok.t {
                            Comma => {
                                match self.lexer.peek().t {
                                    $end_tok => {
                                        self.lexer.next();
                                    },
                                    _ => {exprs.push(self.expr(expects.clone()))}
                                }
                            },
                            $end_tok => {
                                break;
                            }
                            _ => {parser_error!(E; self, tok, vec![Expects::Comma, $end_tok_expect]); break}
                        };
                    };
                }
            };
            exprs
        }
    };
     ($fn_name:ident, [$($end_tok:ident),*]) => {

            list!($fn_name, [$($end_tok; $end_tok),*]);

     };
}

impl<'a> Parser<'a> {
    pub(super) fn expr(&mut self, expects: Vec<Expects>) -> Expr {
        self.add_expr(expects)
    }
    binary_expr!(add_expr, mul_expr,   [Plus     => Add, Minus => Sub]);
    binary_expr!(mul_expr, unary_expr, [Asterisk => Mul, Slash => Div]);
    unary_expr!(primary_expr, [Minus => Neg]);
    // binary_expr!(exp_expr, primary_expr, [])

    list!(fn_args; Expects::RParen; RParen);

    pub(super) fn block(&mut self, expects: Vec<Expects>) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        match_tok_single!(S; self; lb_tok; Expects::LBrace; LBrace => {
            loop {
                let tok = self.lexer.peek();
                if let RBrace = tok.t {
                    self.lexer.next();
                    break;
                } else {
                    stmts.push(self.stmt());
                }
            }
            node!(S; lb_tok, Error)
        });

        stmts
    }
    fn block_expr(&mut self, expects: Vec<Expects>) -> Expr {
        todo!()
    }

    fn primary_expr(&mut self, expects: Vec<Expects>) -> Expr {
        use TokenType::*;
        let tok = self.lexer.next();
        match tok.t {
            IntLiteral(i) => {
                node!(E; tok, Int(i))
            }
            FloatLiteral(f) => {
                node!(E; tok, Float(f))
            }
            LParen => {
                let e = self.expr(expects);
                match_tok_single!(E; self; tok; Expects::RParen; RParen => {
                    e
                })
            }
            Ident(s) => {
                let next_tok = self.lexer.peek();
                match next_tok.t {
                    LParen => {
                        self.lexer.next();
                        let e_list = self.fn_args(expects);
                        node!(E; tok, FnCall(s, e_list))
                    }
                    _ => node!(E; tok, Ident(s)),
                }
            }
            _ => {
                parser_error!(E; self, tok, expects)
            }
        }
    }
}
