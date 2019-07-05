use crate::syntax::ast::*;

type SymbolStack = Vec<Instance>;
type TypeList = Vec<Instance>;

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

    fn extract_type(typed_expr : &Expr) -> Type {
        match typed_expr {
            Expr::Typed(_, type_) => type_.as_ref().clone(),

            _ => panic!("failed to extract types")
        }
    }

    fn check_bind(&mut self, node : Bind) -> Bind {
        match node {
            Bind::Expr(id, expr) => {
                let checked_expr = self.check_expr(expr.as_ref().clone());
                match &checked_expr {
                    Expr::Typed(_, type_) => self.symbols.push(
                        Instance(id.clone(), type_.clone())
                    ),

                    _ => ()
                }
                Bind::Expr(id, Box::new(checked_expr))
            },
            Bind::Type(id, type_) => {
                let checked_type = self.check_type(type_.as_ref().clone());
                self.types.push(Instance(id.clone(), type_.clone()));
                Bind::Type(id, Box::new(checked_type))
            }
        }
    }

    fn check_type(&mut self, node : Type) -> Type {
        match node {
            Type::Sum(instances) => Type::Sum(
                instances.iter().map(|n| self.check_instance(n.clone())).collect()
            ),
            Type::Map(lhs, rhs) => Type::Map(lhs, rhs),
            Type::Var(id) => Type::Var(id)
        }
    }

    fn check_instance(&mut self, instance : Instance) -> Instance {
        self.symbols.push(instance.clone());
        instance
    }

    fn match_apply(f : &Type, x : &Type) -> Option<Type> {
        match f {
            Type::Map(lhs, rhs) => if lhs.as_ref() == x {
                Some(rhs.as_ref().clone())
            } else {
                None
            },

            _ => None
        }
    }

    fn check_expr(&mut self, node : Expr) -> Expr {
        match node {
            Expr::Apply(f, x) => {
                let typed_f = self.check_expr(f.as_ref().clone());
                let typed_x = self.check_expr(x.as_ref().clone());
                match TypeChecker::match_apply(
                    &TypeChecker::extract_type(&typed_f), 
                    &TypeChecker::extract_type(&typed_x)
                ) {
                    Some(t) => Expr::Typed(Box::new(Expr::Apply(
                        Box::new(typed_f), Box::new(typed_x)
                )), Box::new(t)),
                    None => panic!("apply expr: mismatched types")
                }
            },
            Expr::Lambda(patterns) => Expr::Lambda(
                patterns.iter().map(|n| self.check_pattern(n.clone())).collect()
            ), 
            Expr::Typed(expr, type_) => {
                let checked_expr = self.check_expr(expr.as_ref().clone());
                if &TypeChecker::extract_type(&checked_expr) == type_.as_ref() {
                    checked_expr
                } else {
                    panic!("typed expr: mismatched types")
                }
            },
            Expr::Var(id) => {
                match self.symbols.iter().find(|s| s.0 == id) {
                    Some(x) => Expr::Typed(Box::new(Expr::Var(id)), x.1.clone()),
                    None => {
                        panic!(format!("var expr ({:?}): unfound identifiers", id))
                    }
                }
            }
        }
    }

    fn check_pattern(&mut self, node : Pattern) -> Pattern {
        match node.param.as_ref() {
            Expr::Typed(expr, type_) => Pattern {
                param: Box::new(self.infer_expr(expr.as_ref().clone(), type_.as_ref().clone())),
                expr: Box::new(self.check_expr(node.expr.as_ref().clone()))
            },

            _ => unreachable!()
        }
        
    }

    fn comatch_apply(f : &Type, type_ : &Type) -> Option<Type> {
        match f {
            Type::Map(lhs, rhs) => if rhs.as_ref() == type_ {
                Some(lhs.as_ref().clone())
            } else {
                None
            },

            _ => None
        }
    }

    fn infer_expr(&mut self, node : Expr, type_ : Type) -> Expr {
        match node {
            Expr::Apply(f, x) => {
                let typed_f = self.check_expr(f.as_ref().clone());
                let typed_x = match TypeChecker::comatch_apply(
                    &TypeChecker::extract_type(&typed_f), &type_
                ) {
                    Some(t) => self.infer_expr(x.as_ref().clone(), t),
                    None => panic!("apply expr: mismatched types")
                };
                match TypeChecker::match_apply(
                    &TypeChecker::extract_type(&typed_f), 
                    &TypeChecker::extract_type(&typed_x)
                ) {
                    Some(t) => Expr::Typed(Box::new(Expr::Apply(
                        Box::new(typed_f), Box::new(typed_x)
                )), Box::new(t)),
                    None => panic!("apply expr: mismatched types")
                }
            },
            Expr::Lambda(patterns) => Expr::Lambda(
                patterns.iter().map(|n| self.check_pattern(n.clone())).collect()
            ), 
            Expr::Typed(expr, typed_type) => {
                let checked_expr = self.infer_expr(expr.as_ref().clone(), type_);
                if &TypeChecker::extract_type(&checked_expr) == typed_type.as_ref() {
                    checked_expr
                } else {
                    panic!("typed expr: mismatched types")
                }
            },
            Expr::Var(id) => {
                match self.symbols.iter().find(|s| s.0 == id) {
                    Some(x) => Expr::Typed(Box::new(Expr::Var(id)), x.1.clone()),
                    None => {
                        self.symbols.push(Instance(id.clone(), Box::new(type_.clone())));
                        Expr::Typed(Box::new(Expr::Var(id)), Box::new(type_))
                    }
                }
            }
        }
    }
}
