use std::path::Path;

use pest::iterators::Pair;

use crate::{error::ParseError, utils::PairsTrait};

use super::Instruction;
use crate::error::to_static;
use crate::parse_expression;
use crate::rule_to_operator;
use crate::utils::PairTrait;
use crate::Expression;
use crate::{
    function::Function, types::Type, variable::Variable, CompilationContext,
    ParseExpressionContext, Rule,
};

pub fn parse_instructions(
    cvm: Pair<Rule>,
    context: &mut ParseExpressionContext,
) -> Result<Vec<Instruction>> {
    Ok(vec![match cvm.as_rule() {
        Rule::instruction => return parse_instructions(cvm.into_inner().next().unwrap(), context),
        Rule::if_statement => {
            let st = to_static(&cvm);
            let mut inner = cvm.into_inner();
            let expr: Expression = parse_expression(inner.next().unwrap(), context)?;
            Instruction::If(
                st.clone(),
                expr,
                true,
                Expression::VariantAccess(
                    st.clone(),
                    box Expression::Type(st, "Boolean".to_owned()),
                    "true".to_owned(),
                ),
                inner.extract(&mut *context)?,
                if let Some(e) = inner.next() {
                    Some(e.extract(&mut *context)?)
                } else {
                    None
                },
            )
        }
        Rule::expr => Instruction::Expression(to_static(&cvm), parse_expression(cvm, context)?),
        Rule::return_statement => Instruction::Return(
            to_static(&cvm),
            parse_expression(cvm.into_inner().next().unwrap(), context)?,
        ),
        Rule::var_declaration => {
            let st = to_static(&cvm);
            let mut inner = cvm.into_inner();
            let var: Variable = inner.extract(())?;
            context.variables.insert(var.name.clone(), var.clone());
            Instruction::Assign(
                st,
                var,
                parse_expression(inner.skip(1).next().unwrap(), context)?,
            )
        }
        Rule::var_assignement => {
            let st = to_static(&cvm);
            let mut inner = cvm.into_inner();
            let a = inner.next().unwrap();
            let b = inner.next().unwrap();
            let c = inner.next().unwrap();
            let d = inner.next();
            if let Some(d) = d {
                if a.as_rule() == Rule::literal {
                    let a = context
                        .variables
                        .get(a.as_str().trim())
                        .expect("Can't find variable")
                        .clone();
                    Instruction::Assign(
                        st.clone(),
                        a.clone(),
                        Expression::MethodCall(
                            st.clone(),
                            box Expression::Variable(st, a),
                            rule_to_operator(&b.as_rule()).unwrap().to_owned(),
                            vec![parse_expression(d, context)?],
                        ),
                    )
                } else {
                    let mut a = a.into_inner();
                    let i = context
                        .variables
                        .get(&a.extract::<String, ()>(())?)
                        .expect("Can't find variable")
                        .clone();
                    let inside =
                        parse_expression(a.next().unwrap().into_inner().next().unwrap(), context)?;
                    Instruction::Assign(
                        st.clone(),
                        i.clone(),
                        Expression::MethodCall(
                            st.clone(),
                            box Expression::Variable(st.clone(), i.clone()),
                            "replace".to_owned(),
                            vec![Expression::MethodCall(
                                st.clone(),
                                box Expression::MethodCall(
                                    st.clone(),
                                    box Expression::Variable(st, i),
                                    "index".to_owned(),
                                    vec![inside.clone()],
                                ),
                                rule_to_operator(&b.as_rule()).unwrap().to_owned(),
                                vec![parse_expression(d, context)?],
                            )],
                        ),
                    )
                }
            } else {
                if a.as_rule() == Rule::literal {
                    Instruction::Assign(
                        st,
                        context
                            .variables
                            .get(a.as_str().trim())
                            .ok_or_else(|| {
                                ParseError::CantGetVariable(
                                    to_static(&a),
                                    a.as_str().trim().to_owned(),
                                )
                            })?
                            .clone(),
                        parse_expression(c, context)?,
                    )
                } else {
                    let mut a = a.into_inner();
                    let b = context
                        .variables
                        .get(&a.extract::<String, ()>(())?)
                        .expect("Can't find variable")
                        .clone();
                    Instruction::Assign(
                        st.clone(),
                        b.clone(),
                        Expression::Cast(
                            st.clone(),
                            box Expression::MethodCall(
                                st.clone(),
                                box Expression::Variable(st, b.clone()),
                                "replace".to_owned(),
                                vec![
                                    parse_expression(
                                        a.next().unwrap().into_inner().next().unwrap(),
                                        context,
                                    )?,
                                    parse_expression(c, context)?,
                                ],
                            ),
                            b.var_type,
                        ),
                    )
                }
            }
        }
        Rule::loop_statement => {
            Instruction::Loop(to_static(&cvm), cvm.into_inner().extract(context)?)
        }
        Rule::break_instruction => Instruction::Break(to_static(&cvm)),
        Rule::continue_instruction => Instruction::Continue(to_static(&cvm)),
        Rule::comment => return Ok(vec![]),
        Rule::for_statement => {
            let st = to_static(&cvm);
            let mut inner = cvm.into_inner();
            let typed_var: Variable = inner.extract(())?;
            let expr = inner.next().unwrap();
            match expr.as_rule() {
                Rule::range => {
                    let mut expr = expr.into_inner();
                    let expr1 = parse_expression(expr.next().unwrap(), context)?;
                    let expr2 = parse_expression(expr.next().unwrap(), context)?;
                    context
                        .variables
                        .insert(typed_var.name.to_owned(), typed_var.clone());
                    Instruction::ForRange(st, typed_var, expr1, expr2, inner.extract(context)?)
                }
                Rule::expr => {
                    let expr = parse_expression(expr, context)?;
                    context
                        .variables
                        .insert(typed_var.name.to_owned(), typed_var.clone());
                    Instruction::ForEach(st, typed_var, expr, inner.extract(context)?)
                }
                _ => unreachable!(),
            }
        }
        Rule::asm_statement => {
            Instruction::AsmStatement(to_static(&cvm), cvm.into_inner().extract(&mut *context)?)
        }
        e => {
            panic!("Unexpected token in instruction {:?}", e)
        }
    }])
}

type Result<T> = std::result::Result<T, ParseError>;

pub fn parse_type_function(
    cvm: Pair<Rule>,
    context: &mut CompilationContext,
    type_name: &str,
    type_parent: &str,
) -> Result<()> {
    let mut cvm = cvm.into_inner();
    let a = cvm.next().unwrap();
    let is_static = a.as_rule() == Rule::keyword_static;
    if is_static {
        let func: Function = cvm.extract((None, &*context))?;
        context
            .types
            .get_mut(type_name)
            .expect("Can't get type")
            .add_static_function(func);
    } else {
        let func: Function = a.extract((
            Some((type_name.to_owned(), type_parent.to_owned())),
            &*context,
        ))?;
        context
            .types
            .get_mut(type_name)
            .expect("Can't get type")
            .add_function(func);
    };
    Ok(())
}

pub fn file_parser(cvm: Pair<Rule>, context: &mut CompilationContext, file: &Path) -> Result<()> {
    match cvm.as_rule() {
        Rule::file_element => file_parser(cvm.into_inner().next().unwrap(), context, file),
        Rule::file => {
            for x in cvm.into_inner() {
                file_parser(x, context, file)?;
            }
            Ok(())
        }
        Rule::line => file_parser(cvm.into_inner().next().unwrap(), context, file),
        Rule::use_statement => {
            let string: Vec<u8> = cvm.into_inner().extract(())?;
            let string = String::from_utf8(string).unwrap();
            let mut buf = file.to_path_buf();
            buf.pop();
            let mut s: &str = &string;
            while s.starts_with("../") {
                s = &s[3..];
                buf.pop();
            }
            format!("{}.cvm", s).split("/").for_each(|x| buf.push(x));
            crate::compile_file(buf.to_str().unwrap(), context)?;
            Ok(())
        }
        Rule::function => {
            context.add_function(cvm.extract((None, &*context))?);
            Ok(())
        }
        Rule::type_statement => {
            let mut cvm = cvm.into_inner();
            let name = cvm.next().unwrap().as_str().trim();
            context
                .types
                .insert(name.to_owned(), Type::new(name.to_owned()));
            let p = cvm.next().unwrap();
            let p = if Rule::literal == p.as_rule() {
                context.types.get_mut(name).unwrap().parent = p.as_str().trim().to_owned();
                cvm.next().unwrap()
            } else {
                p
            };
            let p = if Rule::equal == p.as_rule() {
                context.types.get_mut(name).unwrap().size =
                    Some(cvm.next().unwrap().as_str().trim().parse().unwrap());
                cvm.next().unwrap()
            } else {
                p
            };
            let parent = &context.types.get_mut(name).unwrap().parent.to_owned();
            parse_type_insides(p, context, name, parent)?;
            for x in cvm {
                parse_type_insides(x, context, name, parent)?;
            }
            Ok(())
        }
        e => {
            panic!("Unexpected {:?} token in file parse", e)
        }
    }
}

pub fn parse_type_insides(
    inside: Pair<Rule>,
    context: &mut CompilationContext,
    name: &str,
    parent: &str,
) -> Result<()> {
    let inside = inside.into_inner().next().unwrap();
    if inside.as_rule() == Rule::type_function {
        parse_type_function(inside, context, name, parent)?;
    } else if inside.as_rule() == Rule::type_ref {
        let mut inside = inside.into_inner();
        let ref_name = inside.next().unwrap().as_str().trim();
        inside.next();
        let ref_value: Vec<u8> = inside.extract(())?;
        context
            .types
            .get_mut(name)
            .unwrap()
            .variants
            .insert(ref_name.to_owned(), ref_value);
    } else {
        panic!("Invalid type_inside token {:?}", inside)
    }
    Ok(())
}
