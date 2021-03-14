#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(associated_type_bounds)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod asm;
pub mod expression;
pub mod function;
pub mod instruction;
pub mod types;
pub mod utils;
pub mod variable;
pub mod cvmir;

use asm::Asm;
use cvmir::{Counter, IrAsm, Optimizer, VariableManager, elide_unused_consts, remove_followed_usages};
use expression::*;
use function::{Function, Functions};
use instruction::file_parser;
use types::Type;
use variable::Variable;

use std::{collections::{HashMap, HashSet}};

use pest::Parser;

const ANY_TYPE: &'static str = "Bytes";
const VOID_TYPE: &'static str = "Empty";
const BYTE_TYPE: &'static str = "Byte";
const CHAR_TYPE: &'static str = "Char";
const STRING_TYPE: &'static str = "String";

#[derive(Parser, Debug)]
#[grammar = "cvm gramar.pest"]
pub struct CVMParser;

#[derive(Default, Debug)]
pub struct CompilationContext {
    pub types: HashMap<String, Type>,
    pub functions: HashMap<String, Functions>,
}

impl CompilationContext {
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

pub struct CVMCompCtx {
    pub ctx: CompilationContext,
    pub current_var: usize,
    pub function: usize,
    pub loops: Vec<usize>,
    pub loop_id: usize,
    pub if_n: usize,
    pub instructions: Vec<IrAsm>,
}

impl CVMCompCtx {
    pub fn new_function(&mut self) -> usize {
        self.function += 1;
        self.function
    }

    pub fn new_loop(&mut self) -> usize {
        self.loop_id += 1;
        self.loop_id
    }

    pub fn new_var(&mut self) -> usize {
        self.current_var += 1;
        self.current_var
    }

    pub fn new_if(&mut self) -> usize {
        self.if_n += 1;
        self.if_n
    }
}

#[derive(Debug)]
pub struct ParseExpressionContext {
    pub variables: HashMap<String, Variable>,
    pub types: HashSet<String>,
}

// Rule::expr

fn main() {
    let file = std::fs::read_to_string("cvm.cvm").unwrap();
    let json = match CVMParser::parse(Rule::line, &file) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    }
    .next()
    .unwrap();
    let mut context = CompilationContext::default();
    // context.types.insert(ANY_TYPE.to_owned(), Type {
    //     functions: HashMap::new(),
    //     allowed_from: Vec::new(),
    //     static_functions: HashMap::new(),
    // });
    file_parser(json, &mut context);

    let func = context
        .functions
        .get("main")
        .unwrap()
        .get_for_input(&Vec::new(), &context)
        .unwrap()
        .clone();
    let mut cctx = CVMCompCtx {
        ctx: context,
        current_var: 0,
        function: 0,
        loops: Vec::new(),
        loop_id: 0,
        if_n: 0,
        instructions: Vec::new(),
    };
    func.compile(&mut cctx, &Vec::new(), None);
    println!("{}",cctx.instructions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
    let mut o = 0;
    while o != cctx.instructions.len() {
        o = cctx.instructions.len();
        cctx.instructions = cctx.instructions.optimize(VariableManager::default()).0;
        cctx.instructions = elide_unused_consts(cctx.instructions, None);
        cctx.instructions = remove_followed_usages(cctx.instructions, None);
    }
    println!("{}",cctx.instructions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
    let mut counter = Counter::default();
    let mut fors = Vec::new();
    let asm: Vec<Asm> = cctx.instructions.into_iter().map(|x| x.to_asm(&mut counter, &mut fors, None)).flatten().collect();
    std::fs::write("new.llasm.cbm", asm.iter().enumerate().map(|(x,y)| format!("l{} {}",x,y)).collect::<Vec<_>>().join("\n")).unwrap();
    let out = Asm::clean(asm)
        .iter()
        .chain(vec![Asm::End].iter())
        .enumerate()
        .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write("new.cbm", &out).unwrap();
    println!("{}", out);
}
