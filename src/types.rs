use std::collections::HashMap;

use crate::{
    function::{Function, Functions},
    CompilationContext, ANY_TYPE,
};

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub size: Option<u8>,
    pub variants: HashMap<String, Vec<u8>>,
    pub functions: HashMap<String, Functions>,
    pub parent: String,
    pub static_functions: HashMap<String, Functions>,
}

impl Type {
    pub fn new(name: String) -> Self {
        Self {
            variants: HashMap::new(),
            name,
            parent: ANY_TYPE.to_owned(),
            size: None,
            functions: HashMap::new(),
            static_functions: HashMap::new(),
        }
    }
}

impl Type {
    pub fn is_child_of(&self, other_type: &str, ctx: &CompilationContext) -> bool {
        if other_type == self.name {
            return true;
        }
        if self.name == ANY_TYPE {
            return false;
        }
        ctx.types
            .get(&self.parent)
            .unwrap()
            .is_child_of(other_type, ctx)
    }

    pub fn add_static_function(&mut self, function: Function) {
        if let Some(e) = self.static_functions.get_mut(&function.name) {
            e.functions.push(function);
        } else {
            self.static_functions.insert(
                function.name.clone(),
                Functions {
                    name: function.name.clone(),
                    functions: vec![function],
                },
            );
        }
    }

    pub fn get_function<'a>(
        &'a self,
        a: &str,
        types: &Vec<String>,
        is_static: bool,
        ctx: &'a CompilationContext,
    ) -> Option<&'a Function> {
        println!("NN: {} {}",a,self.functions.keys().map(|x| x.to_owned()).collect::<Vec<_>>().join(", "));
        if let Some(e) = if is_static {
            &self.static_functions
        } else {
            &self.functions
        }
        .get(a)
        .map(|x| {
            println!("YY: {} {} {:?}",&x.name,a,types);
            if let Some(e) = x.get_for_input(
                &types.iter().map(|x| x.as_str()).collect::<Vec<&str>>(),
                ctx,
            )  {
                //println!("ZZ {:?}",e);
                Some(e)
            } else {
                println!("XX");
                None
            }
        })
        .flatten()
        {
            return Some(e);
        }
        if &self.parent == &self.name {
            println!("Exiting function search due to parent");
            return None;
        }
        println!("Searching in parent {} for {} from {}", self.parent, a,self.name);
        ctx.types
            .get(&self.parent)
            .unwrap()
            .get_function(a, types, is_static, ctx)
    }

    pub fn add_function(&mut self, function: Function) {
        if let Some(e) = self.functions.get_mut(&function.name) {
            e.functions.push(function);
        } else {
            self.functions.insert(
                function.name.clone(),
                Functions {
                    name: function.name.clone(),
                    functions: vec![function],
                },
            );
        }
    }
}
