use std::collections::HashMap;

use crate::{Rule, error::{Paire, ParseError}, types::Type};

use crate::{ANY_TYPE, BYTE_TYPE, CHAR_TYPE, CVMCompCtx, CompilationContext, STRING_TYPE, cvmir::IrAsm, variable::Variable};

#[derive(Clone, Debug)]
pub enum Expression {
    Function(Paire<Rule>, String, Vec<Expression>),
    MethodCall(Paire<Rule>, Box<Expression>, String, Vec<Expression>),
    VariantAccess(Paire<Rule>, Box<Expression>, String),
    // Index(Box<Expression>, Box<Expression>), -> This becomes index(Byte) -> Byte
    Variable(Paire<Rule>, Variable),
    Value(Paire<Rule>, Vec<u8>),
    Type(Paire<Rule>, String),
    Cast(Paire<Rule>, Box<Expression>, String),
}

impl Expression {

    pub fn get_paire(&self) -> &Paire<Rule> {
        match self {
            Self::Function(a,..) |
            Self::MethodCall(a,..) |
            Self::VariantAccess(a,..) |
            Self::Variable(a,..) |
            Self::Value(a,..) |
            Self::Type(a,..) |
            Self::Cast(a,..) => a,
        }
    }

    pub fn get_type<'a>(&'a self, context: &'a CompilationContext) -> Result<&'a Type,ParseError> {
        let e: &'a str = match self {
            Expression::Function(i, name, arguments) => {
                let args: Vec<&'a Type> =
                    arguments.iter().map(|x| x.get_type(context)).collect::<Result<_,_>>()?;
                if let Some(e) = context.functions.get(name) {
                    if let Some(e) = e.get_for_input(&args, context){
                        &e.return_type
                    } else {
                        return Err(ParseError::CantGetFunction(i.clone(),name.to_owned(),args.iter().map(|x| x.name.to_owned()).collect()));
                    }
                } else {
                    return Err(ParseError::CantGetFunction(i.clone(),name.to_owned(),args.iter().map(|x| x.name.to_owned()).collect()));
                }
            }
            Expression::MethodCall(_,expr, method_name, arguments) => {
                let args: Vec<&Type> = arguments
                    .iter()
                    .map(|x| x.get_type(context))
                    .collect::<Result<_,_>>()?;
                let expr_type = expr.get_type(context)?;
                if let Some(e) = expr_type
                    .get_function(method_name, &args, false, context)
                {
                    
                    if &e.return_type == crate::PANIC_TYPE {
                        return Err(ParseError::ForbiddenMethodUse(self.get_paire().clone(),method_name.clone(),expr_type.name.to_owned(),args.iter().map(|x| x.name.to_owned()).collect()));
                    }
                    &e.return_type
                } else {
                    return Err(ParseError::MethodNotFound(self.get_paire().clone(),method_name.clone(),expr_type.name.to_owned(),args.iter().map(|x| x.name.to_owned()).collect()));
                }
            }
            Expression::Variable(_,e) => &e.var_type,
            Expression::Value(_,e) => {
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
            Expression::Type(_,e) => e,
            Expression::Cast(_,_, e) => e,
            Expression::VariantAccess(_,e, _) => return e.get_type(context),
        };
        if e == crate::PANIC_TYPE {
            return Err(ParseError::PanicTypeReached(self.get_paire().clone()));
        }
        if let Some(e) = context
        .types
        .get(e) {
            Ok(e)
        } else {
            Err(ParseError::TypeNotFound(self.get_paire().clone(),e.to_owned()))
        }
    }

    pub fn compile(&self, ctx: &mut CVMCompCtx, vars: &HashMap<String, usize>) -> Result<usize, ParseError> {
        Ok(match self {
            Expression::Function(rule, e, args) => {
                let args_pointer: Vec<usize> = args
                    .iter()
                    .map(|x| {
                        // let var = ctx.new_var();
                        // let data = x.compile(ctx, vars);
                        // ctx.instructions.push(IrAsm::Mov(var, data));
                        // data
                        x.compile(ctx, vars)
                    })
                    .collect::<Result<_, ParseError>>()?;

                    

                        let args_type = args
                        .iter()
                        .map(|x| x.get_type(&ctx.ctx))
                        .collect::<Result<Vec<_>,_>>()?;
                        let func = if let Some(e) = ctx
                    .ctx
                    .functions
                    .get(e) {
                        e
                    } else {
                        return Err(ParseError::CantGetFunction(rule.clone(),e.clone(),args_type.iter().map(|x| x.name.to_owned()).collect()));
                    };
                
                        let func = (*func
                            .get_for_input(
                                &args_type,
                                &ctx.ctx,
                            )
                            .ok_or_else(|| 
                                ParseError::CantGetFunction(rule.clone(), e.clone(), args_type.iter().map(|x| (*x).name.to_owned()).collect())
                            )?)
                        .clone();
                        func.compile(ctx, &args_pointer, None)?
                    }
            Expression::MethodCall(i,a, b, c) => {
                if let box Expression::Type(_,a) = a {
                    let args: Vec<usize> = c
                        .iter()
                        .map(|x| {
                            // let var = ctx.new_var();
                            // let tmp = x.compile(ctx, vars);
                            // ctx.instructions.push(IrAsm::Mov(var, tmp));
                            // var
                            x.compile(ctx, vars)
                        })
                        .collect::<Result<_, ParseError>>()?;
                    let func = (*ctx
                        .ctx
                        .types
                        .get(a)
                        .expect("Can't get type in static method call")
                        .get_function(
                            b,
                            &c.iter().map(|x| x.get_type(&ctx.ctx).map(|x| x.to_owned())).collect::<Result<_,_>>()?,
                            true,
                            &ctx.ctx,
                        )
                        .expect("Can't get static method"))
                    .clone();
                    func.compile(ctx, &args, None)?
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
                        .collect::<Result<_, ParseError>>()?;
                    let compiled = a.compile(ctx, vars)?;
                    // let self_var = ctx.new_var();
                    let on_type = a.get_type(&ctx.ctx)?;
                    let args_type: Vec<&Type> = c.iter().map(|x| x.get_type(&ctx.ctx).map(|x| x.to_owned())).collect::<Result<_,_>>()?;
                    let func = (*on_type
                        .get_function(
                            b,
                            &args_type,
                            false,
                            &ctx.ctx,
                        )
                        .ok_or_else(|| {
                            ParseError::MethodNotFound(i.clone(), b.clone(),on_type.name.clone(), args_type.iter().map(|x| x.name.to_owned()).collect())
                        })?)
                    .clone();
                    // ctx.instructions.push(IrAsm::Mov(self_var, compiled));
                    // func.compile(ctx, &args, Some(self_var))
                    func.compile(ctx, &args, Some(compiled))?
                }
            }
            Expression::Variable(a,e) => {
                *vars.get(&e.name).ok_or_else(|| {
                    ParseError::CantGetVariable(a.clone(),e.name.to_owned())
                })?
            },
            Expression::Value(_,e) => {
                let var = ctx.new_var();
                ctx.instructions.push(IrAsm::Cst(var, e.clone()));
                var
            }
            Expression::Type(e,_) => {
                return Err(ParseError::UnexpectedType(e.clone()));
            }

            Expression::Cast(_,e, _) => e.compile(ctx, vars)?,
            Expression::VariantAccess(a,e, b) => {
                let var = ctx.new_var();
                let i = if let box Expression::Type(_,e) = e {
                    e
                } else {
                    return Err(ParseError::ExpectedTypeInVariant(e.get_paire().clone()));
                };
                ctx.instructions.push(IrAsm::Cst(
                    var,
                    if let Some(e) = ctx.ctx
                        .types
                        .get(i) {
                        if let Some(e) = e.variants.get(b) {
                            e.clone()
                        } else {
                            return Err(ParseError::VariantNotFound(a.clone(),i.clone(),e.name.to_owned()))
                        }
                    } else {
                        return Err(ParseError::TypeNotFound(e.get_paire().clone(),i.clone()))
                    },
                ));
                var
            }
        })
    }
}

fn is_ascii(array: &[u8]) -> bool {
    !array.iter().any(|x| (*x < 32 && *x != 10) || *x > 126)
}
