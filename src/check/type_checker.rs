use crate::syntax::ast::*;

struct Symbol (Ident, Type);
type SymbolStack = Vec<Symbol>;

pub fn check(ast : Main) -> Main {
    let mut stack = SymbolStack::new();
    check_main(ast, &mut stack)
}

fn check_main(node : Main, stack : &mut SymbolStack) -> Main {
    node.into_iter().map(|n| check_bind(n, stack)).collect()
}

fn check_bind(node : Bind, stack : &mut SymbolStack) -> Bind {
    match node {
        Bind::Expr(id, expr) => Bind::Expr(id, 
            Box::new(check_expr(expr.as_ref().clone(), stack))
        ),
        Bind::Type(id, type_) => Bind::Type(id, type_)
    }
}

fn check_expr(node : Expr, stack : &mut SymbolStack) -> Expr {
    match node {
        Expr::Apply(f, x) => Expr::Apply(
            Box::new(check_expr(f.as_ref().clone(), stack)), 
            Box::new(check_expr(x.as_ref().clone(), stack))
        ),
        Expr::Lambda(patterns) => Expr::Lambda(
            patterns.iter().map(|n| check_pattern(n.clone(), stack)).collect()
        ), 
        Expr::Typed(expr, type_) => Expr::Typed(
            Box::new(check_expr(expr.as_ref().clone(), stack)), 
            type_
        ),
        Expr::Var(id) => Expr::Var(id)
    }
}

fn check_pattern(node : Pattern, stack : &mut SymbolStack) -> Pattern {
    Pattern {
        param: Box::new(check_expr(node.param.as_ref().clone(), stack)),
        expr: Box::new(check_expr(node.expr.as_ref().clone(), stack))
    }
}
