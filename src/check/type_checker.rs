use crate::syntax::ast::*;

struct Symbol (Ident, Type);
type SymbolStack = Vec<Symbol>;
type TypeList = Vec<Symbol>;

pub struct TypeChecker {
    symbols : SymbolStack,
    types : TypeList
}

impl TypeChecker {

    pub fn check(ast : Main) -> (Self, Main) {
        let mut type_checker = TypeChecker {
            symbols: SymbolStack::new(),
            types: TypeList::new()
        };

        let typed_ast = type_checker.check_main(ast);
        (type_checker, typed_ast)
    }

    fn check_main(&mut self, node : Main) -> Main {
        node.into_iter().map(|n| self.check_bind(n)).collect()
    }

    fn check_bind(&mut self, node : Bind) -> Bind {
        match node {
            Bind::Expr(id, expr) => Bind::Expr(id, 
                Box::new(self.check_expr(expr.as_ref().clone()))
            ),
            Bind::Type(id, type_) => Bind::Type(id, type_)
        }
    }

    fn check_expr(&mut self, node : Expr) -> Expr {
        match node {
            Expr::Apply(f, x) => Expr::Apply(
                Box::new(self.check_expr(f.as_ref().clone())), 
                Box::new(self.check_expr(x.as_ref().clone()))
            ),
            Expr::Lambda(patterns) => Expr::Lambda(
                patterns.iter().map(|n| self.check_pattern(n.clone())).collect()
            ), 
            Expr::Typed(expr, type_) => Expr::Typed(
                Box::new(self.check_expr(expr.as_ref().clone())), 
                type_
            ),
            Expr::Var(id) => Expr::Var(id)
        }
    }

    fn check_pattern(&mut self, node : Pattern) -> Pattern {
        Pattern {
            param: Box::new(self.check_expr(node.param.as_ref().clone())),
            expr: Box::new(self.check_expr(node.expr.as_ref().clone()))
        }
    }

}