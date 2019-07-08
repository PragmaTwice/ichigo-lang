use crate::syntax::ast::*;

#[derive(Debug)]
pub struct Symbol {
    pub id : Ident,
    pub optional_type : Option<Type>
}

type SymbolStack = Vec<Symbol>;
type TypeList = Vec<Symbol>;
type CheckResult <Node> = Result<Node, String>;
type ParamNumStack = Vec<i32>;

impl Symbol {
    fn new_without_type(id : &Ident) -> Symbol {
        Symbol {
            id : id.clone(),
            optional_type : None
        }
    }

    fn from_instance(instance : &Instance) -> Symbol {
        let Instance(id, type_) = instance;
        Symbol::new_with_box(id, type_)
    }

    fn new_with_box(id: &Ident, type_: &Box<Type>) -> Symbol {
        Symbol {
            id : id.clone(),
            optional_type : Some(type_.as_ref().clone())
        }
    }

    fn set_boxed_type(&mut self, type_: &Box<Type>) {
        if self.optional_type == None {
            self.optional_type = Some(type_.as_ref().clone())
        }
    }
}

#[derive(Debug)]
pub struct TypeChecker {
    pub symbols : SymbolStack,
    pub types : TypeList,
    pub param_num_stack : ParamNumStack
}

impl TypeChecker {

    pub fn check(ast : Main) -> (Self, CheckResult<Main>) {
        let mut type_checker = TypeChecker {
            symbols: SymbolStack::new(),
            types: TypeList::new(),
            param_num_stack: ParamNumStack::new()
        };

        let typed_ast = type_checker.check_main(ast);
        (type_checker, typed_ast)
    }

    fn check_main(&mut self, node : Main) -> CheckResult<Main> {
        node.into_iter().map(|n| self.check_bind(n, None)).collect()
    }

    fn extract_type(typed_expr : &Expr) -> Type {
        match typed_expr {
            Expr::Typed(_, type_) => type_.as_ref().clone(),

            _ => panic!("failed to extract types")
        }
    }

    fn check_bind(&mut self, node : Bind, type_constraint : Option<Type>) -> CheckResult<Bind> {
        match node {
            Bind::Expr(id, expr) => {
                self.symbols.push(Symbol::new_without_type(&id));
                let checked_expr = self.check_expr(expr.as_ref().clone(), type_constraint, false)?;
                match &checked_expr {
                    Expr::Typed(_, type_) => {
                        let pos = self.symbols.iter().position(|s| s.id == id);
                        match pos {
                            Some(n) => self.symbols[n].set_boxed_type(type_),
                            None => ()
                        }
                    },

                    _ => ()
                }
                Ok(Bind::Expr(id, Box::new(checked_expr)))
            },
            Bind::Type(id, type_) => {
                let checked_type = self.check_type(type_.as_ref().clone())?;
                self.types.push(Symbol::new_with_box(&id, &type_));
                Ok(Bind::Type(id, Box::new(checked_type)))
            }
        }
    }

    fn check_type(&mut self, node : Type) -> CheckResult<Type> {
        match node {
            Type::Sum(instances) => Ok(Type::Sum(
                instances.iter().map(|n| self.check_instance(n.clone())).filter_map(Result::ok).collect()
            )),
            Type::Map(lhs, rhs) => Ok(Type::Map(lhs, rhs)),
            Type::Var(id) => Ok(Type::Var(id))
        }
    }

    fn check_instance(&mut self, instance : Instance) -> CheckResult<Instance> {
        self.symbols.push(Symbol::from_instance(&instance));
        Ok(instance)
    }

    fn check_expr(&mut self, node : Expr, type_constraint : Option<Type>, in_param : bool) -> CheckResult<Expr> {
        match node {
            Expr::Apply(f, x) => {
                let try_typed_f = match self.check_expr(f.as_ref().clone(), None, in_param) {
                    Ok(o) => match &TypeChecker::extract_type(&o) {
                        Type::Map(lhs, _) => Some(lhs.as_ref().clone()),

                        _ => None
                    },
                    Err(_) => None
                };
                let typed_x = self.check_expr(x.as_ref().clone(), try_typed_f, in_param)?;
                let typed_f = self.check_expr(f.as_ref().clone(), match type_constraint {
                    Some(f) => Some(
                        Type::Map(
                            Box::new(TypeChecker::extract_type(&typed_x)), 
                            Box::new(f)
                        )
                    ),
                    None => None
                }, in_param)?;
                match {
                    let f = &TypeChecker::extract_type(&typed_f);
                    let x = &TypeChecker::extract_type(&typed_x);
                    match f {
                        Type::Map(lhs, rhs) => if lhs.as_ref() == x {
                            Some(rhs.as_ref().clone())
                        } else {
                            None
                        },

                        _ => None
                    }
                } {
                    Some(t) => Ok(Expr::Typed(
                        Box::new(
                            Expr::Apply(
                                Box::new(typed_f), Box::new(typed_x)
                            )
                        ), Box::new(t)
                    )),
                    None => Err(format!("[apply expr] {:?} {:?} : mismatched types", f, x))
                }
            },
            Expr::Lambda(patterns) => {
                let mut lambda_type : Option<Type> = type_constraint.clone();
                let checked_expr = Expr::Lambda(
                    {
                        let (oks, errs) : (
                            Vec<CheckResult<Pattern>>, 
                            Vec<CheckResult<Pattern>>
                        ) = patterns.iter().map(|n| {
                            let checked_pattern = self.check_pattern(n.clone(), lambda_type.clone())?;
                            let pattern_type = Type::Map(
                                    Box::new(TypeChecker::extract_type(&checked_pattern.param).clone()), 
                                    Box::new(TypeChecker::extract_type(&checked_pattern.expr).clone())
                            );
                            match &lambda_type {
                                Some(lam_t) => if lam_t != &pattern_type {
                                    Err(format!("[lambda expr] {:?} : ambiguous-typed patterns", patterns))
                                } else {
                                    Ok(checked_pattern)
                                },
                                None => {
                                    lambda_type = Some(pattern_type);
                                    Ok(checked_pattern)
                                }
                            }
                        }).partition(Result::is_ok);

                        if errs.is_empty() {
                            oks.into_iter().filter_map(Result::ok).collect()
                        } else {
                            return Err(errs.into_iter().filter_map(Result::err).next().unwrap())
                        }
                    }
                );

                match lambda_type {
                    Some(lam_t) => Ok(Expr::Typed(Box::new(checked_expr), Box::new(lam_t))),
                    None => Err(format!("[lambda expr] {:?} : untyped patterns", patterns))
                }
            }, 
            Expr::Typed(expr, type_) => {
                let checked_expr = self.check_expr(expr.as_ref().clone(), Some(type_.as_ref().clone()), in_param)?;
                if &TypeChecker::extract_type(&checked_expr) == type_.as_ref() {
                    Ok(checked_expr)
                } else {
                    Err(format!("[typed expr] {:?} : {:?} : mismatched types", expr, type_))
                }
            },
            Expr::Var(id) => {
                match self.symbols.iter().find(|s| s.id == id) {
                    Some(s) => match s.optional_type.as_ref() {
                        Some(s_type) => match type_constraint {
                            Some(type_con) => if type_con == *s_type {
                                Ok(Expr::Typed(Box::new(Expr::Var(id)), Box::new(s_type.clone())))
                            } else {
                                Err(format!("[var expr] {:?} : ambiguous-typed identifiers", id))
                            },
                            None => Ok(Expr::Typed(Box::new(Expr::Var(id)), Box::new(s_type.clone())))
                        },
                        None => match type_constraint {
                            Some(type_con) => Ok(Expr::Typed(
                                Box::new(Expr::Var(id)), 
                                Box::new(type_con.clone())
                            )),
                            None => Err(format!("[var expr] {:?} : untyped identifiers", id))
                        }
                    },
                    None => {
                        if !in_param {
                            Err(format!("[var expr] {:?} : unfound identifiers", id))
                        } else {
                            match type_constraint {
                                Some(type_con) => {
                                    self.symbols.push(Symbol {
                                        id: id.clone(), 
                                        optional_type: Some(type_con.clone())
                                    });

                                    let last_num = self.param_num_stack.len() - 1;
                                    self.param_num_stack[last_num] += 1;

                                    Ok(Expr::Typed(
                                        Box::new(Expr::Var(id)), 
                                        Box::new(type_con.clone())
                                    ))
                                },
                                None => Err(format!("[var expr] {:?} : untyped identifiers", id))
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_pattern(&mut self, node : Pattern, type_constraint : Option<Type>) -> CheckResult<Pattern> {
        let (f_type, x_type) = match type_constraint {
            Some(type_) => match type_ {
                Type::Map(lhs, rhs) => (Some(lhs.as_ref().clone()), Some(rhs.as_ref().clone())),

                _ => (None, None)
            },
            None => (None, None)
        };

        self.param_num_stack.push(0);
        let checked_param = self.check_expr(node.param.as_ref().clone(), f_type, true)?;
        let checked_expr = self.check_expr(node.expr.as_ref().clone(), x_type, false)?;

        match self.param_num_stack.pop() {
            Some(n) => for _ in 0..n {
                self.symbols.pop();
            },
            None => ()
        };

        Ok(Pattern {
            param: Box::new(checked_param),
            expr: Box::new(checked_expr)
        })
    }
}
