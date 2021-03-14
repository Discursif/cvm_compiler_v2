use std::collections::HashMap;

use crate::{ANY_TYPE, BYTE_TYPE, CHAR_TYPE, CVMCompCtx, CompilationContext, STRING_TYPE, cvmir::IrAsm, variable::Variable};

#[derive(Clone, Debug)]
pub enum Expression {
    Function(String, Vec<Expression>),
    MethodCall(Box<Expression>, String, Vec<Expression>),
    VariantAccess(Box<Expression>, String),
    // Index(Box<Expression>, Box<Expression>), -> This becomes index(Byte) -> Byte
    Variable(Variable),
    Value(Vec<u8>),
    Type(String),
    Cast(Box<Expression>, String),
}

impl Expression {
    pub fn get_type<'a>(&'a self, context: &'a CompilationContext) -> &'a str {
        match self {
            Expression::Function(name, arguments) => {
                if let Some(e) = context.functions.get(name) {
                    let args: Vec<&'a str> =
                        arguments.iter().map(|x| x.get_type(context)).collect();
                    &e.get_for_input(&args, context)
                        .expect("Can't find function matching type arguments")
                        .return_type
                } else {
                    panic!("Function {} not found", name);
                }
            }
            Expression::MethodCall(expr, method_name, arguments) => {
                let args: Vec<String> = arguments
                    .iter()
                    .map(|x| x.get_type(context).to_owned())
                    .collect();
                let expr_type = expr.get_type(context);
                if let Some(e) = context
                    .types
                    .get(expr_type)
                    .expect("Can't get type for method call")
                    .get_function(method_name, &args, false, context)
                {
                    &e.return_type
                } else {
                    panic!("Function {} not found for type {}", method_name, expr_type);
                }
            }
            Expression::Variable(e) => &e.var_type,
            Expression::Value(e) => {
                if e.len() == 1 {
                    if is_ascii(e) {
                        CHAR_TYPE
                    } else {
                        BYTE_TYPE
                    }
                } else {
                    if is_ascii(e) {
                        STRING_TYPE
                    } else {
                        ANY_TYPE
                    }
                }
            }
            Expression::Type(e) => e,
            Expression::Cast(_, e) => e,
            Expression::VariantAccess(e, _) => e.get_type(context),
        }
    }

    pub fn compile(&self, ctx: &mut CVMCompCtx, vars: &HashMap<String, usize>) -> usize {
        match self {
            Expression::Function(e, args) => {
                let args_pointer: Vec<usize> = args
                    .iter()
                    .map(|x| {
                        // let var = ctx.new_var();
                        // let data = x.compile(ctx, vars);
                        // ctx.instructions.push(IrAsm::Mov(var, data));
                        // data
                        x.compile(ctx, vars)
                    })
                    .collect();

                    let func = if let Some(e) = ctx
                    .ctx
                    .functions
                    .get(e) {
                        e
                    } else {
                        panic!("Function {} not found",e);
                    };
                
                        let func = (*func
                            .get_for_input(
                                &args
                                    .iter()
                                    .map(|x| x.get_type(&ctx.ctx))
                                    .collect::<Vec<_>>(),
                                &ctx.ctx,
                            )
                            .expect("Can't get function"))
                        .clone();
                        func.compile(ctx, &args_pointer, None)
                    }
            Expression::MethodCall(a, b, c) => {
                if let box Expression::Type(a) = a {
                    let args: Vec<usize> = c
                        .iter()
                        .map(|x| {
                            // let var = ctx.new_var();
                            // let tmp = x.compile(ctx, vars);
                            // ctx.instructions.push(IrAsm::Mov(var, tmp));
                            // var
                            x.compile(ctx, vars)
                        })
                        .collect();
                    let func = (*ctx
                        .ctx
                        .types
                        .get(a)
                        .expect("Can't get type in static method call")
                        .get_function(
                            b,
                            &c.iter().map(|x| x.get_type(&ctx.ctx).to_owned()).collect(),
                            true,
                            &ctx.ctx,
                        )
                        .expect("Can't get static method"))
                    .clone();
                    func.compile(ctx, &args, None)
                } else {
                    let args: Vec<usize> = c
                        .iter()
                        .map(|x| {
                            // let var = ctx.new_var();
                            // let data = x.compile(ctx, vars);
                            // ctx.instructions.push(IrAsm::Mov(var, data));
                            // var
                            x.compile(ctx, vars)
                        })
                        .collect();
                    let compiled = a.compile(ctx, vars);
                    // let self_var = ctx.new_var();
                    let func = (*ctx
                        .ctx
                        .types
                        .get(a.get_type(&ctx.ctx))
                        .expect("Can't get type in method call")
                        .get_function(
                            b,
                            &c.iter().map(|x| x.get_type(&ctx.ctx).to_owned()).collect(),
                            false,
                            &ctx.ctx,
                        )
                        .expect("Can't get method"))
                    .clone();
                    // ctx.instructions.push(IrAsm::Mov(self_var, compiled));
                    // func.compile(ctx, &args, Some(self_var))
                    func.compile(ctx, &args, Some(compiled))
                }
            }
            Expression::Variable(e) => *vars.get(&e.name).unwrap(),
            Expression::Value(e) => {
                let var = ctx.new_var();
                ctx.instructions.push(IrAsm::Cst(var, e.clone()));
                var
            }
            Expression::Type(_) => {
                println!("TYPE compile shouldn't be called");
                0
            }

            Expression::Cast(e, _) => e.compile(ctx, vars),
            Expression::VariantAccess(e, b) => {
                let var = ctx.new_var();
                let i = if let box Expression::Type(e) = e {
                    e
                } else {
                    panic!("Variants can only be used on types not expressions {:?}", e)
                };
                ctx.instructions.push(IrAsm::Cst(
                    var,
                    ctx.ctx
                        .types
                        .get(i)
                        .unwrap()
                        .variants
                        .get(b)
                        .unwrap()
                        .clone(),
                ));
                var
            }
        }
    }
}

fn is_ascii(array: &[u8]) -> bool {
    !array.iter().any(|x| (*x < 32 && *x != 10) || *x > 126)
}
