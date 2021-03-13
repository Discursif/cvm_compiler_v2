use crate::{
    utils::{PairTrait, PairsTrait},
    ParseExpressionContext, Rule,
};
use pest::iterators::Pair;

use super::Expression;

pub fn parse_expression(cvm: Pair<Rule>, context: &ParseExpressionContext) -> Expression {
    match cvm.as_rule() {
        Rule::expr => {
            let mut expr = cvm.into_inner();
            let mut ex = parse_expression(expr.next().unwrap(), context);

            for i in expr {
                match i.as_rule() {
                    Rule::operation => {
                        let mut inner = i.into_inner();
                        ex = Expression::MethodCall(
                            Box::new(ex),
                            rule_to_operator(&inner.next().unwrap().as_rule())
                                .unwrap()
                                .to_owned(),
                            vec![parse_expression(inner.next().unwrap(), context)],
                        );
                    }
                    Rule::indexing => {
                        let inner = i.into_inner().next().unwrap();
                        if inner.as_rule() == Rule::expr {
                            ex = Expression::MethodCall(
                                Box::new(ex),
                                "index".to_owned(),
                                vec![parse_expression(inner, context)],
                            );
                        } else {
                            let mut inner = inner.into_inner();
                            ex = Expression::MethodCall(
                                Box::new(ex),
                                "index_range".to_owned(),
                                vec![
                                    parse_expression(inner.next().unwrap(), context),
                                    parse_expression(inner.next().unwrap(), context),
                                ],
                            );
                        }
                    }
                    Rule::variant_access => {
                        ex = Expression::VariantAccess(box ex, i.into_inner().extract(()));
                    }
                    Rule::method_call => {
                        let mut method = i.into_inner();
                        ex = Expression::MethodCall(
                            Box::new(ex),
                            method.extract(()),
                            method
                                .next()
                                .unwrap()
                                .into_inner()
                                .map(|x| parse_expression(x, context))
                                .collect(),
                        );
                    }
                    Rule::literal => {
                        ex = Expression::Cast(box ex, i.as_str().trim().to_owned());
                    }
                    _ => unreachable!(),
                }
            }
            ex
        }
        Rule::literal => {
            let st = cvm.as_str().trim();
            if let Some(e) = context.variables.get(st) {
                Expression::Variable(e.clone())
            } else if context.types.contains(st) {
                Expression::Type(st.to_owned())
            } else {
                panic!("Variable or type `{}` not found\n{}", st, cvm);
            }
        }
        Rule::number_array | Rule::string => Expression::Value(cvm.extract(())),
        Rule::indexing => {
            let mut v = cvm.into_inner();
            Expression::MethodCall(
                Box::new(parse_expression(v.next().unwrap(), context)),
                "index".to_owned(),
                vec![parse_expression(v.next().unwrap(), context)],
            )
        }
        Rule::function_call => {
            let mut v = cvm.into_inner();
            Expression::Function(
                v.extract(()),
                v.next()
                    .unwrap()
                    .into_inner()
                    .map(|x| parse_expression(x, context))
                    .collect(),
            )
        }
        e => {
            panic!("This token wasn't expected in expression {:?}", e)
        }
    }
}

pub fn rule_to_operator(rule: &Rule) -> Option<&'static str> {
    Some(match rule {
        Rule::add => "add",
        Rule::subtract => "sub",
        Rule::multiply => "mul",
        Rule::divide => "div",
        Rule::xor => "xor",
        Rule::merge => "merge",
        _ => return None,
    })
}
