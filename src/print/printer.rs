use crate::syntax::ast::*;

pub fn print(ast : Main) -> String {
    print_main(ast, 0)
}

fn print_main(node : Main, layer : usize) -> String {
    node.into_iter().map(|n| print_bind(n, layer)).collect::<Vec<_>>().join("\n")
}

fn print_bind(node : Bind, layer : usize) -> String {
    match node {
        Bind::Expr(id, expr) => format!("{} = {}", print_ident(id), print_expr(expr.as_ref().clone(), layer)),
        Bind::Type(id, type_) => format!("{} = {}", print_ident(id), print_type(type_.as_ref().clone(), layer))
    }
}

fn print_expr(node : Expr, layer : usize) -> String {
    match node {
        Expr::Apply(lhs, rhs) => format!("({} {})", print_expr(lhs.as_ref().clone(), layer), print_expr(rhs.as_ref().clone(), layer)),
        Expr::Lambda(patterns) => format!("(λ {{\n{}\n{}}})", patterns.into_iter().map(|n| print_pattern(n, layer + 1)).collect::<Vec<_>>().join("\n"), "\t".repeat(layer)),
        Expr::Typed(expr, type_) => format!("({} : {})", print_expr(expr.as_ref().clone(), layer), print_type(type_.as_ref().clone(), layer)),
        Expr::Var(id) => print_ident(id)
    }
}

fn print_type(node : Type, layer : usize) -> String {
    match node {
        Type::Map(lhs, rhs) => format!("({} → {})", print_type(lhs.as_ref().clone(), layer), print_type(rhs.as_ref().clone(), layer)),
        Type::Sum(instances) => format!("σ {{\n{}\n{}}}", instances.into_iter().map(|n| print_instance(n, layer + 1)).collect::<Vec<_>>().join("\n"), "\t".repeat(layer)),
        Type::Var(id) => print_ident(id)
    }
}

fn print_instance(node : Instance, layer : usize) -> String {
    format!("{}{} : {}", "\t".repeat(layer), print_ident(node.0), print_type(node.1.as_ref().clone(), layer))
}

fn print_pattern(node : Pattern, layer : usize) -> String {
    format!("{}{} . {}", "\t".repeat(layer), print_expr(node.param.as_ref().clone(), layer), print_expr(node.expr.as_ref().clone(), layer))
}

fn print_ident(ident : Ident) -> String {
    ident.0.clone()
}
