use crate::{
    error::{to_static, ParseError},
    utils::{PairTrait, PairsTrait},
    ParseExpressionContext, Rule,
};
use pest::iterators::Pair;

use super::Expression;

type Result<T> = std::result::Result<T, ParseError>;

pub fn parse_expression<'a>(
    cvm: Pair<'a, Rule>,
    context: &ParseExpressionContext,
) -> Result<Expression> {
    Ok(match cvm.as_rule() {
        Rule::expr => {
            let mut expr = cvm.into_inner();
            let pom = expr.next().unwrap();
            let mut ex = parse_expression(expr.next().unwrap(), context)?;
            if pom.as_rule() == Rule::not {
                ex = Expression::MethodCall(to_static(&pom), box ex, "not".to_owned(), vec![]);
            }
            for i in expr {
                match i.as_rule() {
                    Rule::operation => {
                        let st = to_static(&i);
                        let mut inner = i.into_inner();
                        ex = Expression::MethodCall(
                            st,
                            Box::new(ex),
                            rule_to_operator(&inner.next().unwrap().as_rule())
                                .unwrap()
                                .to_owned(),
                            vec![parse_expression(inner.next().unwrap(), context)?],
                        );
                    }
                    Rule::indexing => {
                        let st = to_static(&i);
                        let inner = i.into_inner().next().unwrap();
                        if inner.as_rule() == Rule::expr {
                            ex = Expression::MethodCall(
                                st,
                                Box::new(ex),
                                "index".to_owned(),
                                vec![parse_expression(inner, context)?],
                            );
                        } else {
                            let mut inner = inner.into_inner();
                            ex = Expression::MethodCall(
                                st,
                                Box::new(ex),
                                "index_range".to_owned(),
                                vec![
                                    parse_expression(inner.next().unwrap(), context)?,
                                    parse_expression(inner.next().unwrap(), context)?,
                                ],
                            );
                        }
                    }
                    Rule::variant_access => {
                        let st = to_static(&i);
                        ex = Expression::VariantAccess(st, box ex, i.into_inner().extract(())?);
                    }
                    Rule::method_call => {
                        let st = to_static(&i);
                        let mut method = i.into_inner();
                        ex = Expression::MethodCall(
                            st,
                            Box::new(ex),
                            method.extract(())?,
                            method
                                .next()
                                .unwrap()
                                .into_inner()
                                .map(|x| parse_expression(x, context))
                                .collect::<Result<_>>()?,
                        );
                    }
                    Rule::literal => {
                        ex = Expression::Cast(to_static(&i), box ex, i.as_str().trim().to_owned());
                    }
                    _ => unreachable!(),
                }
            }
            ex
        }
        Rule::literal => {
            let st = cvm.as_str().trim();
            if let Some(e) = context.variables.get(st) {
                Expression::Variable(to_static(&cvm), e.clone())
            } else if context.types.contains(st) {
                Expression::Type(to_static(&cvm), st.to_owned())
            } else {
                return Err(ParseError::CantFindVariableOrType(
                    to_static(&cvm),
                    st.to_owned(),
                ));
            }
        }
        Rule::number_array | Rule::string => Expression::Value(to_static(&cvm), cvm.extract(())?),
        Rule::indexing => {
            let st = to_static(&cvm);
            let mut v = cvm.into_inner();
            Expression::MethodCall(
                st.clone(),
                Box::new(parse_expression(v.next().unwrap(), context)?),
                "index".to_owned(),
                vec![parse_expression(v.next().unwrap(), context)?],
            )
        }
        Rule::function_call => {
            let st = to_static(&cvm);
            let mut v = cvm.into_inner();
            Expression::Function(
                st,
                v.extract(())?,
                v.next()
                    .unwrap()
                    .into_inner()
                    .map(|x| parse_expression(x, context))
                    .collect::<Result<_>>()?,
            )
        }
        e => {
            panic!("This token wasn't expected in expression {:?}", e)
        }
    })
}

pub fn rule_to_operator(rule: &Rule) -> Option<&'static str> {
    Some(match rule {
        Rule::add => "add",
        Rule::subtract => "sub",
        Rule::multiply => "mul",
        Rule::modulo => "mod",
        Rule::divide => "div",
        Rule::xor => "xor",
        Rule::merge => "merge",
        Rule::lower => "lower",
        Rule::greater => "greater",
        Rule::greater_equals => "greater_equals",
        Rule::boolean_and => "double_and",
        Rule::boolean_or => "double_or",
        Rule::double_equal => "equals",
        Rule::not_equal => "not_equals",
        Rule::and => "and",
        Rule::or => "or",
        _ => return None,
    })
}
