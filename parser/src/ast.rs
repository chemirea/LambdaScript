#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    DefStmt(Def),
}

#[derive(Debug, PartialEq)]
pub struct Def {
    pub name: Identifier,
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(Identifier),
    Abstraction(Abstruction),
    Application(Application),
    Literal(String),
}

pub type Identifier = String;

#[derive(Debug, PartialEq)]
pub struct Abstruction {
    pub args: Args,
    pub body: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Application {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

pub type Args = Vec<Identifier>;

#[derive(Debug, PartialEq)]
pub struct Literal(pub String);
