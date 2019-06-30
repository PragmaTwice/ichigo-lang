#[derive(Debug)]
pub struct Ident {
    pub name : String
}

#[derive(Debug)]
pub struct Pattern {
    pub param : Box<Expr>,
    pub expr : Box<Expr>
}

#[derive(Debug)]
pub enum Expr {
    Var(Ident),
    Lambda(Vec<Pattern>, Box<Type>),
    Apply(Box<Self>, Box<Self>),
    Typed(Box<Self>, Box<Type>)
}

#[derive(Debug)]
pub enum Type {
    Var(Ident),
    Map(Box<Self>, Box<Self>),
    Sum(Vec<Instance>)
}

#[derive(Debug)]
pub struct Instance {
    pub ins : Ident,
    pub type_ : Box<Type>
}

#[derive(Debug)]
pub enum Bind {
    Expr(Ident, Box<Expr>),
    Type(Ident, Box<Type>)
}
