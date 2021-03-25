use std::collections::HashMap;

use crate::IrAsm;
use crate::Rule;
use crate::{
    asm::OperationType,
    error::{Paire, ParseError},
    expression::Expression,
    variable::Variable,
    CVMCompCtx, BYTE_TYPE,
};

#[derive(Clone, Debug)]
pub enum AsmInstruction {
    Mov(AsmVariable, AsmVariable),
    Operation(OperationType, AsmVariable, AsmVariable, AsmVariable),
    Len(AsmVariable, AsmVariable),
    Read(AsmVariable, AsmVariable, AsmVariable, AsmVariable),
    Print(AsmVariable),
    Input(AsmVariable),
    Return(AsmVariable),
    Const(AsmVariable, Vec<u8>),
    NoOp,
    End,
    Continue,
    Break,
    If(
        AsmVariable,
        AsmVariable,
        Vec<AsmInstruction>,
        Vec<AsmInstruction>,
    ),
}

#[derive(Clone, Debug)]
pub enum AsmVariable {
    External(String),
    Internal(String),
    TypedExternal(Variable),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    AsmStatement(Paire<Rule>, Vec<AsmInstruction>),
    If(
        Paire<Rule>,
        Expression,
        bool,
        Expression,
        Vec<Instruction>,
        Option<Vec<Instruction>>,
    ),
    Expression(Paire<Rule>, Expression),
    ForEach(Paire<Rule>, Variable, Expression, Vec<Instruction>),
    ForRange(
        Paire<Rule>,
        Variable,
        Expression,
        Expression,
        Vec<Instruction>,
    ),
    Loop(Paire<Rule>, Vec<Instruction>),
    Break(Paire<Rule>),
    Continue(Paire<Rule>),
    Assign(Paire<Rule>, Variable, Expression),
    Return(Paire<Rule>, Expression),
}

impl Instruction {
    pub fn compile(
        &self,
        ctx: &mut CVMCompCtx,
        function_data: &String, /* Function id, Function return var location, Function return type */
        vars: &mut HashMap<String, usize>,
    ) -> Result<(), ParseError> {
        match self {
            Instruction::If(_, a, b, c, d, e) => {
                //let expr1 = ctx.new_var();
                let expr1_r = a.compile(ctx, vars)?;
                //ctx.instructions.push(IrAsm::Mov(expr1, expr1_r));
                //let expr2 = ctx.new_var();
                let expr2_r = c.compile(ctx, vars)?;
                //ctx.instructions.push(IrAsm::Mov(expr2, expr2_r));
                let tmp = ctx.instructions.clone();
                ctx.instructions = Vec::new();
                for x in d {
                    x.compile(ctx, function_data, vars)?;
                }
                let d = ctx.instructions.clone();
                ctx.instructions = Vec::new();
                if let Some(e) = e {
                    for x in e {
                        x.compile(ctx, function_data, vars)?;
                    }
                }
                let e = ctx.instructions.clone();
                ctx.instructions = tmp;
                let (a, b) = if *b { (d, e) } else { (e, d) };
                //ctx.instructions.push(IrAsm::If(expr1, expr2, a,b));
                ctx.instructions.push(IrAsm::If(expr1_r, expr2_r, a, b));
            }
            Instruction::Expression(_, e) => {
                e.compile(ctx, vars)?;
            }
            Instruction::Assign(g, e, a) => {
                let expr1 = vars.get(&e.name).cloned().unwrap_or_else(|| ctx.new_var());
                let p = a.get_type(&ctx.ctx)?;
                if !p.is_child_of(&e.var_type, &ctx.ctx) {
                    return Err(ParseError::InvalidAssignement(
                        g.clone(),
                        e.var_type.to_owned(),
                        p.name.to_owned(),
                    ));
                }
                // I'm not sure if I can remove this MOV since if we do
                // a = "test"
                // b = a
                // b = "tast"
                // print(a) -> "tast"
                let expr1_r = a.compile(ctx, vars)?;
                ctx.instructions.push(IrAsm::Mov(expr1, expr1_r));
                vars.insert(e.name.to_owned(), expr1);
            }
            Instruction::Return(a, e) => {
                let tp = e.get_type(&ctx.ctx)?;
                if !tp.is_child_of(&function_data, &ctx.ctx) {
                    return Err(ParseError::InvalidReturnType(
                        a.clone(),
                        function_data.clone(),
                        tp.name.to_owned(),
                    ));
                }
                let expr1_r = e.compile(ctx, vars)?;
                ctx.instructions.push(IrAsm::Return(expr1_r));
            }
            Instruction::Loop(_, e) => {
                let tmp = ctx.instructions.clone();
                ctx.instructions = Vec::new();
                for i in e.iter() {
                    i.compile(ctx, function_data, vars)?;
                }
                let out = ctx.instructions.clone();
                ctx.instructions = tmp;
                ctx.instructions.push(IrAsm::Loop(out));
            }
            Instruction::Break(_) => {
                ctx.instructions.push(IrAsm::Break());
            }
            Instruction::Continue(_) => {
                ctx.instructions.push(IrAsm::Continue());
            }
            Instruction::ForEach(e, a, b, c) => {
                let for_id = ctx.new_if();
                let b_type = b.get_type(&ctx.ctx)?;
                let object = Variable {
                    name: format!("~foreach_object_{}", for_id),
                    var_type: b_type.name.to_owned(),
                };
                let objectlen = Variable {
                    name: format!("~foreach_objectlen_{}", for_id),
                    var_type: BYTE_TYPE.to_owned(),
                };
                let index = Variable {
                    name: format!("~foreach_index_{}", for_id),
                    var_type: BYTE_TYPE.to_owned(),
                };
                Instruction::Assign(e.clone(), object.clone(), b.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                )?;
                Instruction::Assign(
                    e.clone(),
                    objectlen.clone(),
                    Expression::MethodCall(
                        e.clone(),
                        box Expression::Variable(e.clone(), object.clone()),
                        "len".to_owned(),
                        vec![],
                    ),
                )
                .compile(ctx, function_data, vars)?;
                Instruction::Assign(
                    e.clone(),
                    index.clone(),
                    Expression::Value(e.clone(), vec![0]),
                )
                .compile(ctx, function_data, vars)?;
                Instruction::Loop(e.clone(), {
                    let mut ol = Vec::new();
                    ol.push(Instruction::If(
                        e.clone(),
                        Expression::Variable(e.clone(), objectlen.clone()),
                        true,
                        Expression::Variable(e.clone(), index.clone()),
                        vec![Instruction::Break(e.clone())],
                        None,
                    ));
                    ol.push(Instruction::Assign(
                        e.clone(),
                        a.clone(),
                        Expression::MethodCall(
                            e.clone(),
                            box Expression::Variable(e.clone(), object.clone()),
                            "index".to_owned(),
                            vec![Expression::Variable(e.clone(), index.clone())],
                        ),
                    ));
                    ol.push(Instruction::Assign(
                        e.clone(),
                        index.clone(),
                        Expression::MethodCall(
                            e.clone(),
                            box Expression::Variable(e.clone(), index.clone()),
                            "add".to_owned(),
                            vec![Expression::Value(e.clone(), vec![1])],
                        ),
                    ));
                    ol.append(&mut c.clone());
                    ol
                })
                .compile(ctx, function_data, vars)?;
            }
            Instruction::ForRange(e, typed_var, expr1, expr2, code) => {
                let for_id = ctx.new_if();
                let expr1_var = Variable {
                    name: format!("~for_range1_{}", for_id),
                    var_type: expr1.get_type(&ctx.ctx)?.name.to_owned(),
                };
                let expr2_var = Variable {
                    name: format!("~for_range2_{}", for_id),
                    var_type: expr2.get_type(&ctx.ctx)?.name.to_owned(),
                };
                Instruction::Assign(e.clone(), expr1_var.clone(), expr1.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                )?;
                Instruction::Assign(e.clone(), expr2_var.clone(), expr2.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                )?;
                Instruction::Loop(e.clone(), {
                    let mut lo = Vec::new();
                    lo.push(Instruction::If(
                        e.clone(),
                        Expression::Variable(e.clone(), expr1_var.clone()),
                        true,
                        Expression::Variable(e.clone(), expr2_var.clone()),
                        vec![Instruction::Break(e.clone())],
                        None,
                    ));
                    lo.push(Instruction::Assign(
                        e.clone(),
                        typed_var.clone(),
                        Expression::Variable(e.clone(), expr1_var.clone()),
                    ));
                    lo.push(Instruction::Assign(
                        e.clone(),
                        expr1_var.clone(),
                        Expression::MethodCall(
                            e.clone(),
                            box Expression::Variable(e.clone(), expr1_var.clone()),
                            "add".to_owned(),
                            vec![Expression::Value(e.clone(), vec![1])],
                        ),
                    ));
                    lo.append(&mut code.clone());
                    lo
                })
                .compile(ctx, function_data, vars)?;
            }
            Instruction::AsmStatement(_, e) => {
                let mut refs: HashMap<String, usize> = HashMap::new();
                let mut i = {
                    e.iter()
                        .map(|i| compile(i, vars, &mut refs, ctx))
                        .collect::<Vec<_>>()
                };
                ctx.instructions.append(&mut i);
            }
        }
        Ok(())
    }
}

fn compile(
    i: &AsmInstruction,
    vars: &mut HashMap<String, usize>,
    refs: &mut HashMap<String, usize>,
    ctx: &mut CVMCompCtx,
) -> IrAsm {
    let mut var = |v: &AsmVariable| -> usize {
        match v {
            AsmVariable::External(e) => *vars.get(e).unwrap(),
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
    match i {
        AsmInstruction::Mov(e, a) => IrAsm::Mov(var(e), var(a)),
        AsmInstruction::Operation(a, b, c, d) => IrAsm::Op(a.clone(), var(b), var(c), var(d)),
        AsmInstruction::Len(a, b) => IrAsm::Len(var(a), var(b)),
        AsmInstruction::Read(a, b, c, d) => IrAsm::Read(var(a), var(b), var(c), var(d)),
        AsmInstruction::Print(e) => IrAsm::Prt(var(e)),
        AsmInstruction::Input(e) => IrAsm::Inp(var(e)),
        AsmInstruction::NoOp => IrAsm::Nop,
        AsmInstruction::End => IrAsm::End,
        AsmInstruction::If(a, b, c, d) => IrAsm::If(
            var(a),
            var(b),
            c.iter().map(|x| compile(x, vars, refs, ctx)).collect(),
            d.iter().map(|x| compile(x, vars, refs, ctx)).collect(),
        ),
        AsmInstruction::Const(e, a) => IrAsm::Cst(var(e), a.clone()),
        AsmInstruction::Return(e) => IrAsm::Return(var(e)),
        AsmInstruction::Continue => IrAsm::Continue(),
        AsmInstruction::Break => IrAsm::Break(),
    }
}
