use std::collections::HashMap;

use crate::{
    instruction::Instruction, variable::Variable, CVMCompCtx, CompilationContext, ANY_TYPE,
};

use crate::asm::Asm;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Variable>,
    pub return_type: String,
    pub code: Vec<Instruction>,
}

impl Function {
    pub fn compile(&self, ctx: &mut CVMCompCtx, pos: &[usize], self_pos: Option<usize>) -> usize {
        let id = ctx.new_function();
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
        for i in &self.code {
            i.compile(ctx, &(id, return_var, self.return_type.clone()), &mut vars);
        }
        ctx.instructions.push(Asm::Label(format!("fn_end_{}", id)));
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
