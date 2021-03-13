use crate::CompilationContext;

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub var_type: String,
}

impl Variable {
    pub fn is_child_of(&self, other_type: &str, ctx: &CompilationContext) -> bool {
        ctx.types
            .get(&self.var_type)
            .unwrap()
            .is_child_of(other_type, ctx)
    }
}
