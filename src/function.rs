use std::collections::HashMap;

use crate::{ANY_TYPE, CVMCompCtx, CompilationContext, cvmir::IrAsm, instruction::Instruction, variable::Variable};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Variable>,
    pub return_type: String,
    pub code: Vec<Instruction>,
}

impl Function {
    pub fn compile(&self, ctx: &mut CVMCompCtx, pos: &[usize], self_pos: Option<usize>) -> usize {
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
            i.compile(ctx, &self.return_type, &mut vars);
        }
        let out = ctx.instructions.clone();
        ctx.instructions = tmp;
        ctx.instructions.push(IrAsm::FunctionBlock(return_var, out));
        return_var
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
        arguments: &[&str],
        context: &CompilationContext,
    ) -> Option<&'a Function> {
        self.functions.iter().find(|x| {
            if x.arguments.len() != arguments.len() {
                return false;
            }
            !x.arguments.iter().zip(arguments.iter()).any(|(x, y)| {
                !(x.var_type == ANY_TYPE
                    || &x.var_type == y
                    || context.types.get(*y).unwrap().is_child_of(&x.name, context))
            })
        })
    }
}
