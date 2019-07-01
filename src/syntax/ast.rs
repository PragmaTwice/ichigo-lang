#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident (
    pub String
);

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub param : Box<Expr>,
    pub expr : Box<Expr>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(Ident),
    Lambda(Vec<Pattern>),
    Apply(Box<Self>, Box<Self>),
    Typed(Box<Self>, Box<Type>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Var(Ident),
    Map(Box<Self>, Box<Self>),
    Sum(Vec<Instance>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance (
    pub Ident,
    pub Box<Type>
);

#[derive(Debug, Clone, PartialEq)]
pub enum Bind {
    Expr(Ident, Box<Expr>),
    Type(Ident, Box<Type>)
}

pub type Main = Vec<Bind>;
