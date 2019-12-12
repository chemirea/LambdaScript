pub type Program = Vec<Stmt>;

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
    Abstruction(Abstruction),
    Application(Application),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Identifier(String),
    ExternIdentifier(String),
}

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

pub type Literal = String;
