use std::{collections::HashMap, fmt::Display};

use crate::{ANY_TYPE, CVMCompCtx, CompilationContext, cvmir::IrAsm, error::ParseError, instruction::Instruction, types::Type, variable::Variable};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Variable>,
    pub return_type: String,
    pub code: Vec<Instruction>,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}) -> {}",self.name, self.arguments.iter().map(|x| format!("{} {}",x.var_type,x.name)).collect::<Vec<_>>().join(", "),self.return_type)
    }
}

impl Function {
    pub fn compile(&self, ctx: &mut CVMCompCtx, pos: &[usize], self_pos: Option<usize>) -> Result<usize, ParseError> {
        let return_var = ctx.new_var();
        let mut vars = HashMap::new();
        vars.extend(
            self.arguments
                .iter()
                .zip(pos.iter())
                .map(|(x, y)| (x.name.to_owned(), *y)),
        );
        if let Some(e) = self_pos {
            vars.insert("self".to_owned(), e);
            vars.insert("super".to_owned(), e);
        }
        let tmp = ctx.instructions.clone();
        ctx.instructions = Vec::new();
        for i in &self.code {
            i.compile(ctx, &self.return_type, &mut vars)?;
        }
        let out = ctx.instructions.clone();
        ctx.instructions = tmp;
        ctx.instructions.push(IrAsm::FunctionBlock(return_var, out));
        Ok(return_var)
    }
}

#[derive(Debug)]
pub struct Functions {
    pub name: String,
    pub functions: Vec<Function>,
}

impl Functions {
    pub fn get_for_input<'a>(
        &'a self,
        arguments: &[&Type],
        context: &CompilationContext,
    ) -> Option<&'a Function> {
        self.functions.iter().find(|x| {
            if x.arguments.len() != arguments.len() {
                return false;
            }
            for (x,y) in x.arguments.iter().zip(arguments.iter()) {
                if x.var_type == ANY_TYPE || &x.var_type == &y.name || y.is_child_of(&x.name, context) {
                    continue;
                }
                return false;
            }
            true
        })
    }
}
