use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use crate::{instruction::{AsmInstruction, AsmVariable}, parse_expression};
use crate::{
    expression::Expression, function::Function, instruction::parse_instructions,
    CompilationContext, VOID_TYPE,
};
use crate::asm::OperationType;
use crate::{instruction::Instruction, variable::Variable, ParseExpressionContext, Rule};

pub trait TryFromRule<E> {
    fn extract(rule: Pair<Rule>, e: E) -> Self;
}

pub trait PairTrait {
    fn extract<T: TryFromRule<E>, E>(self, e: E) -> T;
}
pub trait PairsTrait {
    fn extract<T: TryFromRule<E>, E>(&mut self, e: E) -> T;
}

impl PairTrait for Pair<'_, Rule> {
    fn extract<T: TryFromRule<E>, E>(self, e: E) -> T {
        T::extract(self, e)
    }
}

impl PairsTrait for Pairs<'_, Rule> {
    fn extract<T: TryFromRule<E>, E>(&mut self, e: E) -> T {
        T::extract(self.next().unwrap(), e)
    }
}

impl TryFromRule<()> for Variable {
    fn extract(rule: Pair<Rule>, _: ()) -> Self {
        match rule.as_rule() {
            Rule::typed_var => {
                let mut def = rule.into_inner();
                Self {
                    var_type: def.extract(()),
                    name: def.extract(()),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<()> for Vec<Variable> {
    fn extract(rule: Pair<Rule>, _: ()) -> Self {
        match rule.as_rule() {
            Rule::functions_params => rule.into_inner().map(|x| x.extract(())).collect(),
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<()> for String {
    fn extract(rule: Pair<Rule>, _: ()) -> Self {
        match rule.as_rule() {
            Rule::literal => rule.as_str().trim().to_owned(),
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<()> for Vec<u8> {
    fn extract(rule: Pair<Rule>, _: ()) -> Self {
        match rule.as_rule() {
            Rule::string => rule
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .as_bytes()
                .to_vec(),
            Rule::number_array => rule
                .into_inner()
                .map(|x| x.as_str().trim().parse().expect("Not a u8"))
                .collect(),
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<&mut ParseExpressionContext> for Vec<Instruction> {
    fn extract(rule: Pair<Rule>, context: &mut ParseExpressionContext) -> Self {
        match rule.as_rule() {
            Rule::code_block => rule
                .into_inner()
                .flat_map(|x| parse_instructions(x, context))
                .collect(),
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<(Option<(String,String)>, &CompilationContext)> for Function {
    fn extract(
        rule: Pair<Rule>,
        (in_type, context): (Option<(String,String)>, &CompilationContext),
    ) -> Self {
        match rule.as_rule() {
            Rule::function => {
                let mut cvm = rule.into_inner();
                let name: String = cvm.extract(());
                let arguments: Vec<Variable> = cvm.extract(());
                let c = cvm.next().unwrap();
                let (return_type, c) = if c.as_rule() == Rule::literal {
                    (c.as_str().trim().to_owned(), cvm.next().unwrap())
                } else {
                    (VOID_TYPE.to_owned(), c)
                };
                let mut context = ParseExpressionContext {
                    types: context.types.iter().map(|(x, _)| x.to_owned()).collect(),
                    variables: {
                        let mut map = HashMap::new();
                        if let Some((e,parent)) = in_type {
                            map.insert(
                                "self".to_owned(),
                                Variable {
                                    name: "self".to_owned(),
                                    var_type: e,
                                },
                            );
                            map.insert(
                                "super".to_owned(),
                                Variable {
                                    name: "super".to_owned(),
                                    var_type: parent,
                                },
                            );
                        }
                        map.extend(arguments.iter().map(|x| (x.name.to_owned(), x.clone())));
                        map
                    },
                };
                Function {
                    name,
                    arguments,
                    return_type,
                    code: c.extract(&mut context),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<&ParseExpressionContext> for (Expression, bool, Expression) {
    fn extract(rule: Pair<Rule>, context: &ParseExpressionContext) -> Self {
        match rule.as_rule() {
            Rule::boolean_test => {
                let mut boolean_test = rule.into_inner();
                let inverted = boolean_test.next().unwrap().as_rule() == Rule::not;
                let a = boolean_test.next().unwrap();
                let a = parse_expression(a, context);
                let b = boolean_test
                    .next()
                    .map(|x| {
                        if inverted {
                            x.as_rule() != Rule::double_equal
                        } else {
                            x.as_rule() == Rule::double_equal
                        }
                    })
                    .unwrap_or(!inverted);
                let c = boolean_test
                    .next()
                    .map(|x| parse_expression(x, context))
                    .unwrap_or_else(|| {
                        Expression::VariantAccess(
                            box Expression::Type("Boolean".to_owned()),
                            "true".to_owned(),
                        )
                    });
                (a, b, c)
            }
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<&mut ParseExpressionContext> for AsmVariable {
    fn extract(rule: Pair<Rule>, ctx: &mut ParseExpressionContext) -> Self {
        match rule.as_rule() {
            Rule::asm_variable => rule.into_inner().extract(ctx),
            Rule::asm_local => {
                Self::Internal(rule.into_inner().next().unwrap().extract(()))
            }
            Rule::literal => {
                Self::External(rule.extract(()))
            }
            Rule::typed_var => {
                let var: Variable = rule.extract(());
                ctx.variables.insert(var.name.clone(), var.clone());
                Self::TypedExternal(var)
            }
            _ => unreachable!(),
        }
    }
}

impl TryFromRule<&mut ParseExpressionContext> for AsmInstruction {
    fn extract(rule: Pair<Rule>, ctx: &mut ParseExpressionContext) -> Self {
        match rule.as_rule() {
            Rule::asm_instruction => rule.into_inner().extract(&mut *ctx),
            Rule::i_four => {
                let mut po = rule.into_inner();
                match po.next().unwrap().as_str().trim() {
                    "READ" => Self::Read(po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    _ => unreachable!()
                }
            }
            Rule::i_three => {
                let mut po = rule.into_inner();
                match po.next().unwrap().as_str().trim() {
                    "ADD" => Self::Operation(OperationType::Add,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "SUB" => Self::Operation(OperationType::Sub,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "MUL" => Self::Operation(OperationType::Mul,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "DIV" => Self::Operation(OperationType::Div,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "XOR" => Self::Operation(OperationType::Xor,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "MERGE" => Self::Operation(OperationType::Merge,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "AND" => Self::Operation(OperationType::And,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "MOD" => Self::Operation(OperationType::Mod,po.extract(&mut *ctx),po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    _ => unreachable!()
                }
            }
            Rule::i_two => {
                let mut po = rule.into_inner();
                match po.next().unwrap().as_str().trim() {
                    "MOV" => Self::Mov(po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "IF" => Self::If(po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "IFN" => Self::IfN(po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    "LEN" => Self::Len(po.extract(&mut *ctx),po.extract(&mut *ctx)),
                    _ => unreachable!()
                }
            }
            Rule::i_one => {
                let mut po = rule.into_inner();
                match po.next().unwrap().as_str().trim() {
                    "INPUT" => Self::Input(po.extract(&mut *ctx)),
                    "PRINT" => Self::Print(po.extract(&mut *ctx)),
                    _ => unreachable!()
                }
            }
            Rule::i_zero => {
                match rule.as_str().trim() {
                    "END" => Self::End,
                    "NOOP" => Self::NoOp,
                    _ => unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}
