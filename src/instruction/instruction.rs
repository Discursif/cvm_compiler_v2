use std::collections::HashMap;

use crate::{BYTE_TYPE, CVMCompCtx, asm::{Asm, OperationType}, expression::Expression, variable::Variable};

#[derive(Clone, Debug)]
pub enum AsmInstruction {
    Mov(AsmVariable,AsmVariable),
    Operation(OperationType,AsmVariable,AsmVariable,AsmVariable),
    Len(AsmVariable,AsmVariable),
    Read(AsmVariable,AsmVariable,AsmVariable,AsmVariable),
    Print(AsmVariable),
    Input(AsmVariable),
    NoOp,
    End,
    If(AsmVariable,AsmVariable),
    IfN(AsmVariable,AsmVariable),
}

#[derive(Clone, Debug)]
pub enum AsmVariable {
    External(String),
    Internal(String),
    TypedExternal(Variable)
}

#[derive(Debug, Clone)]
pub enum Instruction {
    AsmStatement(
        Vec<AsmInstruction>
    ),
    If(
        Expression,
        bool,
        Expression,
        Vec<Instruction>,
        Option<Vec<Instruction>>,
    ),
    Expression(Expression),
    ForEach(Variable, Expression, Vec<Instruction>),
    ForRange(Variable, Expression, Expression, Vec<Instruction>),
    Loop(Vec<Instruction>),
    Break,
    Continue,
    Assign(Variable, Expression),
    Return(Expression),
}

impl Instruction {
    pub fn compile(
        &self,
        ctx: &mut CVMCompCtx,
        function_data: &(usize, usize, String), /* Function id, Function return var location, Function return type */
        vars: &mut HashMap<String, usize>,
    ) {
        match self {
            Instruction::If(a, b, c, d, e) => {
                let if_id = ctx.new_if();
                let expr1 = ctx.new_var();
                let expr1_r = a.compile(ctx, vars);
                ctx.instructions.push(Asm::Mov(expr1, expr1_r));
                let expr2 = ctx.new_var();
                let expr2_r = c.compile(ctx, vars);
                ctx.instructions.push(Asm::Mov(expr2, expr2_r));
                ctx.instructions.push(Asm::If(!*b, expr1, expr2));
                ctx.instructions
                    .push(Asm::GtLabel(format!("if_else_{}", if_id)));
                d.iter().for_each(|x| x.compile(ctx, function_data, vars));
                ctx.instructions
                    .push(Asm::GtLabel(format!("if_end_{}", if_id)));
                ctx.instructions
                    .push(Asm::Label(format!("if_else_{}", if_id)));
                if let Some(e) = e {
                    e.iter().for_each(|x| x.compile(ctx, function_data, vars));
                }
                ctx.instructions
                    .push(Asm::Label(format!("if_end_{}", if_id)));
            }
            Instruction::Expression(e) => {
                e.compile(ctx, vars);
            }
            Instruction::Assign(e, a) => {
                let expr1 = vars.get(&e.name).cloned().unwrap_or_else(|| ctx.new_var());
                let p = a.get_type(&ctx.ctx);
                if !ctx
                    .ctx
                    .types
                    .get(p)
                    .unwrap()
                    .is_child_of(&e.var_type, &ctx.ctx)
                {
                    panic!("Can't assign a {} with {}", e.var_type, p);
                }
                let expr1_r = a.compile(ctx, vars);
                ctx.instructions.push(Asm::Mov(expr1, expr1_r));
                vars.insert(e.name.to_owned(), expr1);
            }
            Instruction::Return(e) => {
                if !ctx
                    .ctx
                    .types
                    .get(e.get_type(&ctx.ctx))
                    .unwrap()
                    .is_child_of(&function_data.2, &ctx.ctx)
                {
                    panic!("Invalid return type {:?}", e)
                }
                let expr1_r = e.compile(ctx, vars);
                ctx.instructions.push(Asm::Mov(function_data.1, expr1_r));
                ctx.instructions
                    .push(Asm::GtLabel(format!("fn_end_{}", function_data.0)));
            }
            Instruction::Loop(e) => {
                let loop_id = ctx.new_loop();
                ctx.loops.push(loop_id);
                ctx.instructions
                    .push(Asm::Label(format!("loop_start_{}", loop_id)));
                e.iter().for_each(|x| x.compile(ctx, function_data, vars));
                ctx.instructions
                    .push(Asm::GtLabel(format!("loop_start_{}", loop_id)));
                ctx.instructions
                    .push(Asm::Label(format!("loop_end_{}", loop_id)));
                ctx.loops.pop();
            }
            Instruction::Break => {
                ctx.instructions.push(Asm::GtLabel(format!(
                    "loop_end_{}",
                    ctx.loops.last().unwrap()
                )));
            }
            Instruction::Continue => {
                ctx.instructions.push(Asm::GtLabel(format!(
                    "loop_start_{}",
                    ctx.loops.last().unwrap()
                )));
            }
            Instruction::ForEach(a, b, c) => {
                let for_id = ctx.new_if();
                let b_type = b.get_type(&ctx.ctx);
                let object = Variable {
                    name: format!("~foreach_object_{}", for_id),
                    var_type: b_type.to_owned(),
                };
                let objectlen = Variable {
                    name: format!("~foreach_objectlen_{}", for_id),
                    var_type: BYTE_TYPE.to_owned(),
                };
                let index = Variable {
                    name: format!("~foreach_index_{}", for_id),
                    var_type: BYTE_TYPE.to_owned(),
                };
                Instruction::Assign(object.clone(), b.clone()).compile(ctx, function_data, vars);
                Instruction::Assign(
                    objectlen.clone(),
                    Expression::MethodCall(
                        box Expression::Variable(object.clone()),
                        "len".to_owned(),
                        vec![],
                    ),
                )
                .compile(ctx, function_data, vars);
                Instruction::Assign(index.clone(), Expression::Value(vec![0])).compile(
                    ctx,
                    function_data,
                    vars,
                );
                Instruction::Loop({
                    let mut ol = Vec::new();
                    ol.push(Instruction::If(
                        Expression::Variable(objectlen.clone()),
                        true,
                        Expression::Variable(index.clone()),
                        vec![Instruction::Break],
                        None,
                    ));
                    ol.push(Instruction::Assign(
                        a.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(object.clone()),
                            "index".to_owned(),
                            vec![Expression::Variable(index.clone())],
                        ),
                    ));
                    ol.push(Instruction::Assign(
                        index.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(index.clone()),
                            "add".to_owned(),
                            vec![Expression::Value(vec![1])],
                        ),
                    ));
                    ol.append(&mut c.clone());
                    ol
                })
                .compile(ctx, function_data, vars);
            }
            Instruction::ForRange(typed_var, expr1, expr2, code) => {
                let for_id = ctx.new_if();
                let expr1_var = Variable {
                    name: format!("~for_range1_{}", for_id),
                    var_type: expr1.get_type(&ctx.ctx).to_owned(),
                };
                let expr2_var = Variable {
                    name: format!("~for_range2_{}", for_id),
                    var_type: expr2.get_type(&ctx.ctx).to_owned(),
                };
                Instruction::Assign(expr1_var.clone(), expr1.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                );
                Instruction::Assign(expr2_var.clone(), expr2.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                );
                Instruction::Loop({
                    let mut lo = Vec::new();
                    lo.push(Instruction::If(
                        Expression::Variable(expr1_var.clone()),
                        true,
                        Expression::Variable(expr2_var.clone()),
                        vec![Instruction::Break],
                        None,
                    ));
                    lo.push(Instruction::Assign(
                        typed_var.clone(),
                        Expression::Variable(expr1_var.clone()),
                    ));
                    lo.push(Instruction::Assign(
                        expr1_var.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(expr1_var.clone()),
                            "add".to_owned(),
                            vec![Expression::Value(vec![1])],
                        ),
                    ));
                    lo.append(&mut code.clone());
                    lo
                })
                .compile(ctx, function_data, vars);
            }
            Instruction::AsmStatement(e) => {
                let mut refs: HashMap<String, usize> = HashMap::new();
                let mut i = {
                    let mut var = |v: &AsmVariable| -> usize {
                        match v {
                            AsmVariable::External(e) => {
                                *vars.get(e).unwrap()
                            }
                            AsmVariable::Internal(e) => {
                                if let Some(e) = refs.get(e) {
                                    *e
                                } else {
                                    let i = ctx.new_var();
                                    refs.insert(e.clone(), i);
                                    i
                                }
                            }
                            AsmVariable::TypedExternal(e) => {
                                let i = ctx.new_var();
                                vars.insert(e.name.clone(), i);
                                i
                            }
                        }
                    };
                    e.iter().map(|i| {
                        match i {
                            AsmInstruction::Mov(e, a) => Asm::Mov(var(e),var(a)),
                            AsmInstruction::Operation(a, b, c, d) => Asm::Op(a.clone(),var(b),var(c),var(d)),
                            AsmInstruction::Len(a, b) => Asm::Len(var(a),var(b)),
                            AsmInstruction::Read(a, b, c, d) => Asm::Read(var(a),var(b),var(c),var(d)),
                            AsmInstruction::Print(e) => Asm::Prt(var(e)),
                            AsmInstruction::Input(e) => Asm::Inp(var(e)),
                            AsmInstruction::NoOp => Asm::Nop,
                            AsmInstruction::End => Asm::End,
                            AsmInstruction::If(e, a) => Asm::If(true,var(e),var(a)),
                            AsmInstruction::IfN(e, a) => Asm::If(false,var(e),var(a)),
                        }
                    }).collect::<Vec<_>>()
                };
                ctx.instructions.append(&mut i);
            }
        }
    }
}
