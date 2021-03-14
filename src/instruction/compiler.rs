use pest::iterators::Pair;

use crate::utils::PairsTrait;

use super::Instruction;
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
) -> Vec<Instruction> {
    vec![match cvm.as_rule() {
        Rule::instruction => return parse_instructions(cvm.into_inner().next().unwrap(), context),
        Rule::if_statement => {
            let mut inner = cvm.into_inner();
            let (a, b, c) = inner.extract(&*context);
            Instruction::If(
                a,
                b,
                c,
                inner.extract(&mut *context),
                inner.next().map(|y| y.extract(&mut *context)),
            )
        }
        Rule::expr => Instruction::Expression(parse_expression(cvm, context)),
        Rule::return_statement => {
            Instruction::Return(parse_expression(cvm.into_inner().next().unwrap(), context))
        }
        Rule::var_declaration => {
            let mut inner = cvm.into_inner();
            let var: Variable = inner.extract(());
            context.variables.insert(var.name.clone(), var.clone());
            Instruction::Assign(
                var,
                parse_expression(inner.skip(1).next().unwrap(), context),
            )
        }
        Rule::var_assignement => {
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
                        a.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(a),
                            rule_to_operator(&b.as_rule()).unwrap().to_owned(),
                            vec![parse_expression(d, context)],
                        ),
                    )
                } else {
                    let mut a = a.into_inner();
                    let i = context
                        .variables
                        .get(&a.extract::<String, ()>(()))
                        .expect("Can't find variable")
                        .clone();
                    let inside = parse_expression(a.next().unwrap(), context);
                    Instruction::Assign(
                        i.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(i.clone()),
                            "replace".to_owned(),
                            vec![Expression::MethodCall(
                                box Expression::MethodCall(
                                    box Expression::Variable(i),
                                    "index".to_owned(),
                                    vec![inside.clone()],
                                ),
                                rule_to_operator(&b.as_rule()).unwrap().to_owned(),
                                vec![parse_expression(d, context)],
                            )],
                        ),
                    )
                }
            } else {
                if a.as_rule() == Rule::literal {
                    Instruction::Assign(
                        context
                            .variables
                            .get(a.as_str().trim())
                            .expect("Can't find variable")
                            .clone(),
                        parse_expression(c, context),
                    )
                } else {
                    let mut a = a.into_inner();
                    let b = context
                        .variables
                        .get(&a.extract::<String, ()>(()))
                        .expect("Can't find variable")
                        .clone();
                    Instruction::Assign(
                        b.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(b),
                            "replace".to_owned(),
                            vec![parse_expression(a.next().unwrap(), context)],
                        ),
                    )
                }
            }
        }
        Rule::loop_statement => Instruction::Loop(cvm.into_inner().extract(context)),
        Rule::break_instruction => Instruction::Break,
        Rule::continue_instruction => Instruction::Continue,
        Rule::comment => return vec![],
        Rule::for_statement => {
            let mut inner = cvm.into_inner();
            let typed_var: Variable = inner.extract(());
            let expr = inner.next().unwrap();
            match expr.as_rule() {
                Rule::range => {
                    let mut expr = expr.into_inner();
                    let expr1 = parse_expression(expr.next().unwrap(), context);
                    let expr2 = parse_expression(expr.next().unwrap(), context);
                    context
                        .variables
                        .insert(typed_var.name.to_owned(), typed_var.clone());
                    Instruction::ForRange(typed_var, expr1, expr2, inner.extract(context))
                }
                Rule::expr => {
                    let expr = parse_expression(expr, context);
                    context
                        .variables
                        .insert(typed_var.name.to_owned(), typed_var.clone());
                    Instruction::ForEach(typed_var, expr, inner.extract(context))
                }
                _ => unreachable!(),
            }
        }
        Rule::asm_statement => {
            Instruction::AsmStatement(cvm.into_inner().map(|x| x.extract(&mut *context)).collect())
        }
        e => {
            panic!("Unexpected token in instruction {:?}", e)
        }
    }]
}

pub fn parse_type_function(cvm: Pair<Rule>, context: &mut CompilationContext, type_name: &str, type_parent: &str) {
    let mut cvm = cvm.into_inner();
    let a = cvm.next().unwrap();
    let is_static = a.as_rule() == Rule::keyword_static;
    if is_static {
        let func: Function = cvm.extract((None, &*context));
        context
            .types
            .get_mut(type_name)
            .expect("Can't get type")
            .add_static_function(func);
    } else {
        let func: Function = a.extract((Some((type_name.to_owned(), type_parent.to_owned())), &*context));
        context
            .types
            .get_mut(type_name)
            .expect("Can't get type")
            .add_function(func);
    };
}

pub fn file_parser(cvm: Pair<Rule>, context: &mut CompilationContext) {
    match cvm.as_rule() {
        Rule::file_element => file_parser(cvm.into_inner().next().unwrap(), context),
        Rule::file => cvm.into_inner().for_each(|x| file_parser(x, context)),
        Rule::line => file_parser(cvm.into_inner().next().unwrap(), context),
        Rule::use_statement => {
            println!("Uses aren't implemented yet");
        }
        Rule::function => {
            context.add_function(cvm.extract((None, &*context)));
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
            parse_type_insides(p, context, name,parent);
            cvm.for_each(|x| parse_type_insides(x, context, name,parent));
        }
        e => {
            panic!("Unexpected {:?} token in file parse", e)
        }
    }
}

pub fn parse_type_insides(inside: Pair<Rule>, context: &mut CompilationContext, name: &str,parent: &str) {
    let inside = inside.into_inner().next().unwrap();
    if inside.as_rule() == Rule::type_function {
        parse_type_function(inside, context, name,parent);
    } else if inside.as_rule() == Rule::type_ref {
        let mut inside = inside.into_inner();
        let ref_name = inside.next().unwrap().as_str().trim();
        inside.next();
        let ref_value: Vec<u8> = inside.extract(());
        context
            .types
            .get_mut(name)
            .unwrap()
            .variants
            .insert(ref_name.to_owned(), ref_value);
        println!("REFERING1");
    } else {
        panic!("Invalid type_inside token {:?}", inside)
    }
}
