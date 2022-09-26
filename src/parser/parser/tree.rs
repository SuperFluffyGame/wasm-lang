pub type Program = Vec<Stmt>;

#[derive(Debug)]
pub enum StmtType {
    Error,

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
pub enum ExprType {
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

pub struct Stmt {
    pub t: StmtType,
    pub line: i32,
    pub col: i32,
}
impl std::fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:Stmt->{:?}", self.line, self.col, self.t)
    }
}
impl Stmt {
    pub fn new(t: StmtType, line: i32, col: i32) -> Self {
        Self { t, line, col }
    }
}
pub struct Expr {
    pub t: ExprType,
    pub line: i32,
    pub col: i32,
}
impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:Expr->{:?}", self.line, self.col, self.t)
    }
}
impl Expr {
    pub fn new(t: ExprType, line: i32, col: i32) -> Self {
        Self { t, line, col }
    }
}
