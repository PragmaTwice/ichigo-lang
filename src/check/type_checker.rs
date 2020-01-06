use crate::syntax::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: Ident,
    pub optional_type: Option<Type>,
    pub instance_from: Option<Ident>,
}

type SymbolStack = Vec<Symbol>;
type TypeStack = Vec<Ident>;
type CheckResult<Node> = Result<Node, String>;
type ParamNumStack = Vec<i32>;

impl Symbol {
    fn new(id: &Ident) -> Self {
        Self {
            id: id.clone(),
            optional_type: None,
            instance_from: None,
        }
    }

    pub fn new_with_type(id: &Ident, type_: &Type) -> Self {
        Self {
            id: id.clone(),
            optional_type: Some(type_.clone()),
            instance_from: None,
        }
    }

    fn from_instance(instance: &Instance, from_type: &Ident) -> Self {
        let Instance(id, type_) = instance;

        Self {
            id: id.clone(),
            optional_type: Some(type_.as_ref().clone()),
            instance_from: Some(from_type.clone()),
        }
    }
}

#[derive(Debug)]
pub struct TypeChecker {
    pub symbols: SymbolStack,
    pub types: TypeStack,
    pub param_num_stack: ParamNumStack,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbols: SymbolStack::new(),
            types: TypeStack::new(),
            param_num_stack: ParamNumStack::new(),
        }
    }

    pub fn check(&mut self, ast: Main) -> CheckResult<Main> {
        self.check_main(ast)
    }

    #[allow(dead_code)]
    pub fn from_check(ast: Main) -> (Self, CheckResult<Main>) {
        let mut type_checker = Self::new();
        let typed_ast = type_checker.check(ast);
        (type_checker, typed_ast)
    }

    fn check_main(&mut self, node: Main) -> CheckResult<Main> {
        node.into_iter().map(|n| self.check_bind(n, None)).collect()
    }

    fn extract_type(typed_expr: &Expr) -> Type {
        match typed_expr {
            Expr::Typed(_, type_) => type_.as_ref().clone(),

            _ => panic!("failed to extract types"),
        }
    }

    fn check_bind(&mut self, node: Bind, type_constraint: Option<Type>) -> CheckResult<Bind> {
        match node {
            Bind::Expr(id, expr) => {
                if self.symbols.iter().any(|s| s.id == id) {
                    return Err(format!("[bind expr] {:?} : redefined symbols", id));
                }

                self.symbols.push(Symbol::new(&id));

                let checked_expr =
                    match self.check_expr(expr.as_ref().clone(), type_constraint, false) {
                        Ok(o) => o,
                        Err(e) => {
                            self.symbols.pop();
                            return Err(e);
                        }
                    };

                let symbol = self.symbols.last_mut().unwrap();
                let type_ = Self::extract_type(&checked_expr);

                symbol.optional_type = Some(type_);

                Ok(Bind::Expr(id, Box::new(checked_expr)))
            }
            Bind::Type(id, type_) => {
                if self.types.contains(&id) {
                    return Err(format!("[bind type] {:?} : redefined types", id));
                }

                self.types.push(id.clone());

                let checked_type = match self.check_type(type_.as_ref().clone()) {
                    Ok(o) => o,
                    Err(e) => {
                        let last_type = self.types.pop();

                        self.symbols.retain(|s| s.instance_from != last_type);

                        return Err(e);
                    }
                };

                Ok(Bind::Type(id, Box::new(checked_type)))
            }
        }
    }

    fn check_type(&mut self, node: Type) -> CheckResult<Type> {
        match node {
            Type::Sum(instances) => {
                let (oks, errs): (Vec<CheckResult<Instance>>, Vec<CheckResult<Instance>>) =
                    instances
                        .iter()
                        .map(|n| self.check_instance(n.clone()))
                        .partition(Result::is_ok);

                if errs.is_empty() {
                    Ok(Type::Sum(oks.into_iter().filter_map(Result::ok).collect()))
                } else {
                    Err(errs.into_iter().filter_map(Result::err).collect())
                }
            }
            Type::Map(lhs, rhs) => {
                let checked_lhs = self.check_type(lhs.as_ref().clone())?;
                let checked_rhs = self.check_type(rhs.as_ref().clone())?;
                Ok(Type::Map(Box::new(checked_lhs), Box::new(checked_rhs)))
            }
            Type::Var(id) => {
                if self.types.contains(&id) {
                    Ok(Type::Var(id))
                } else {
                    Err(format!("[var type] {:?} : unfound types", id))
                }
            }
        }
    }

    fn check_instance(&mut self, instance: Instance) -> CheckResult<Instance> {
        let checked_type = self.check_type(instance.1.as_ref().clone())?;
        let checked_instance = Instance(instance.0.clone(), Box::new(checked_type));

        if self.symbols.iter().any(|s| s.id == instance.0.clone()) {
            return Err(format!("[instance type] {:?} : redefined symbols", instance.0.clone()));
        }

        self.symbols.push(Symbol::from_instance(
            &checked_instance,
            self.types.last().unwrap(),
        ));
        Ok(checked_instance)
    }

    fn check_expr(
        &mut self,
        node: Expr,
        type_constraint: Option<Type>,
        in_param: bool,
    ) -> CheckResult<Expr> {
        match node {
            Expr::Apply(f, x) => {
                let try_typed_f = match self.check_expr(f.as_ref().clone(), None, in_param) {
                    Ok(o) => match &Self::extract_type(&o) {
                        Type::Map(lhs, _) => Some(lhs.as_ref().clone()),

                        _ => None,
                    },
                    Err(_) => None,
                };
                let typed_x = self.check_expr(x.as_ref().clone(), try_typed_f, in_param)?;
                let typed_f = self.check_expr(
                    f.as_ref().clone(),
                    match type_constraint {
                        Some(f) => Some(Type::Map(
                            Box::new(Self::extract_type(&typed_x)),
                            Box::new(f),
                        )),
                        None => None,
                    },
                    in_param,
                )?;
                match {
                    let f = &Self::extract_type(&typed_f);
                    let x = &Self::extract_type(&typed_x);
                    match f {
                        Type::Map(lhs, rhs) => {
                            if lhs.as_ref() == x {
                                Some(rhs.as_ref().clone())
                            } else {
                                None
                            }
                        }

                        _ => None,
                    }
                } {
                    Some(t) => Ok(Expr::Typed(
                        Box::new(Expr::Apply(Box::new(typed_f), Box::new(typed_x))),
                        Box::new(t),
                    )),
                    None => Err(format!("[apply expr] {:?} {:?} : mismatched types", f, x)),
                }
            }
            Expr::Lambda(patterns) => {
                let mut lambda_type: Option<Type> = type_constraint.clone();
                let checked_expr = Expr::Lambda({
                    let (oks, errs): (Vec<CheckResult<Pattern>>, Vec<CheckResult<Pattern>>) =
                        patterns
                            .iter()
                            .map(|n| {
                                let checked_pattern =
                                    self.check_pattern(n.clone(), lambda_type.clone())?;
                                let pattern_type = Type::Map(
                                    Box::new(Self::extract_type(&checked_pattern.param).clone()),
                                    Box::new(Self::extract_type(&checked_pattern.expr).clone()),
                                );
                                match &lambda_type {
                                    Some(lam_t) => {
                                        if lam_t != &pattern_type {
                                            Err(format!(
                                                "[lambda expr] {:?} : ambiguous-typed patterns",
                                                patterns
                                            ))
                                        } else {
                                            Ok(checked_pattern)
                                        }
                                    }
                                    None => {
                                        lambda_type = Some(pattern_type);
                                        Ok(checked_pattern)
                                    }
                                }
                            })
                            .partition(Result::is_ok);

                    if errs.is_empty() {
                        oks.into_iter().filter_map(Result::ok).collect()
                    } else {
                        return Err(errs.into_iter().filter_map(Result::err).next().unwrap());
                    }
                });

                match lambda_type {
                    Some(lam_t) => Ok(Expr::Typed(Box::new(checked_expr), Box::new(lam_t))),
                    None => Err(format!("[lambda expr] {:?} : untyped patterns", patterns)),
                }
            }
            Expr::Typed(expr, type_) => {
                let checked_type = self.check_type(type_.as_ref().clone())?;

                let checked_expr =
                    self.check_expr(expr.as_ref().clone(), Some(checked_type), in_param)?;

                if &Self::extract_type(&checked_expr) == type_.as_ref() {
                    Ok(checked_expr)
                } else {
                    Err(format!(
                        "[typed expr] {:?} : {:?} : mismatched types",
                        expr, type_
                    ))
                }
            }
            Expr::Var(id) => match self.symbols.iter().find(|s| s.id == id) {
                Some(s) => match s.optional_type.as_ref() {
                    Some(s_type) => match type_constraint {
                        Some(type_con) => {
                            if type_con == *s_type {
                                Ok(Expr::Typed(
                                    Box::new(Expr::Var(id)),
                                    Box::new(s_type.clone()),
                                ))
                            } else {
                                Err(format!("[var expr] {:?} : ambiguous-typed identifiers", id))
                            }
                        }
                        None => Ok(Expr::Typed(
                            Box::new(Expr::Var(id)),
                            Box::new(s_type.clone()),
                        )),
                    },
                    None => match type_constraint {
                        Some(type_con) => Ok(Expr::Typed(
                            Box::new(Expr::Var(id)),
                            Box::new(type_con.clone()),
                        )),
                        None => Err(format!("[var expr] {:?} : untyped identifiers", id)),
                    },
                },
                None => {
                    if !in_param {
                        Err(format!("[var expr] {:?} : unfound identifiers", id))
                    } else {
                        match type_constraint {
                            Some(type_con) => {
                                self.symbols.push(Symbol::new_with_type(&id, &type_con));

                                let last_num = self.param_num_stack.len() - 1;
                                self.param_num_stack[last_num] += 1;

                                Ok(Expr::Typed(
                                    Box::new(Expr::Var(id)),
                                    Box::new(type_con.clone()),
                                ))
                            }
                            None => Err(format!("[var expr] {:?} : untyped identifiers", id)),
                        }
                    }
                }
            },
        }
    }

    fn check_pattern(
        &mut self,
        node: Pattern,
        type_constraint: Option<Type>,
    ) -> CheckResult<Pattern> {
        let (f_type, x_type) = match type_constraint {
            Some(type_) => match type_ {
                Type::Map(lhs, rhs) => (Some(lhs.as_ref().clone()), Some(rhs.as_ref().clone())),

                _ => (None, None),
            },
            None => (None, None),
        };

        self.param_num_stack.push(0);

        let result = match self.check_expr(node.param.as_ref().clone(), f_type, true) {
            Ok(checked_param) => match self.check_expr(node.expr.as_ref().clone(), x_type, false) {
                Ok(checked_expr) => Ok(Pattern {
                    param: Box::new(checked_param),
                    expr: Box::new(checked_expr),
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        };

        match self.param_num_stack.pop() {
            Some(n) => {
                for _ in 0..n {
                    self.symbols.pop();
                }
            }
            None => (),
        }

        result
    }
}
