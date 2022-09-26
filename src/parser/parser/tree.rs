pub type Program = Vec<Stmt>;

#[derive(Debug)]
pub enum Stmt {
    FnDecl(
        // name
        Expr,
        // param names
        Vec<Expr>,
        // param types
        Vec<Expr>,
        // body statements
        Vec<Stmt>,
    ),
    Let(
        // name
        String,
        // value
        Expr,
    ),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Error,

    Ident(String),
    Type(String),
    Int(i64),
    Float(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),

    FnCall(String, Vec<Expr>),
}
