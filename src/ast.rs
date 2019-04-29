use std::string;

pub struct Ident {
    pub name : String
}

pub struct Pattern {
    pub param : Expr
    pub type : Type
    pub expr : Expr
}

pub enum Expr {
    Var(Ident),
    Lambda(Vec<Pattern>),
    Apply(Self, Self),
    Typed(Self, Type)
}

pub enum Type {
    Var(Ident),
    Map(Self, Self)
}

pub struct Instance {
    pub ins : Ident,
    pub type : Type
}

pub enum Bind {
    Expr(Ident, Expr),
    TypeDef(Ident, Vec<Instance>)
}
