pub type Program = Vec<Stmt>;

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

pub enum Expr {
    Ident(String),
    Type(String),
    Int(i64),
    Float(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    FnCall(String, Vec<Expr>),
}
