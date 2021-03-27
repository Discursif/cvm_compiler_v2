#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(associated_type_bounds)]
#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod asm;
pub mod cvmir;
pub mod error;
pub mod expression;
pub mod function;
pub mod instruction;
pub mod types;
pub mod utils;
pub mod variable;
pub mod cvm_exe;
pub mod cli;

use asm::Asm;
use cvm_exe::{clean_asm_to_exe, exe_to_clean_asm};
use cvmir::{Counter, IrAsm, computer, elide_unused_writes, fn_inliner, if_cleaner, loop_break_inline, remap_consts, remove_followed_usages};
use error::ParseError;
use expression::*;
use function::{Function, Functions};
use instruction::file_parser;
use path_absolutize::Absolutize;
use types::Type;
use variable::Variable;

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use pest::Parser;

const ANY_TYPE: &'static str = "Bytes";
const VOID_TYPE: &'static str = "Empty";
const BYTE_TYPE: &'static str = "Byte";
const CHAR_TYPE: &'static str = "Char";
const PANIC_TYPE: &'static str = "Panic";
const STRING_TYPE: &'static str = "String";

#[derive(Parser, Debug)]
#[grammar = "cvm gramar.pest"]
pub struct CVMParser;

#[derive(Default, Debug)]
pub struct CompilationContext {
    pub files: HashSet<String>,
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

pub fn compile_file(file: &str, context: &mut CompilationContext) -> Result<(), ParseError> {
    if context.files.contains(file) {
        return Ok(());
    }
    context.files.insert(file.to_owned());
    let path = Path::new(file).absolutize().unwrap();
    let file = std::fs::read_to_string(&path).unwrap();
    let json = match CVMParser::parse(Rule::line, &file) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    }
    .next()
    .unwrap();
    file_parser(json, context, path.as_ref())
}

fn main() {
    let mut context = CompilationContext::default();
    match compile_file("cvm/src/game.cvm", &mut context) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
        }
    };

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
    match func.compile(&mut cctx, &Vec::new(), None) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    };
    std::fs::write(
        "cvm/build/new.mlasm.cbm",
        cctx.instructions
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    let mut o = 0;
    let mut p = 0;
    loop {
        if o != cctx.instructions.len() {
            p = 0;
        } else {
            p += 1;
            if p == 3 {
                break;
            }
        }
        o = cctx.instructions.len();
        //cctx.instructions = cctx.instructions.optimize(VariableManager::default()).0;
        cctx.instructions = elide_unused_writes(cctx.instructions);
        cctx.instructions = remove_followed_usages(cctx.instructions, None);
        cctx.instructions = cvmir::regroup_consts::optimize(cctx.instructions);
        cctx.instructions = computer::optimize(cctx.instructions);
        cctx.instructions = fn_inliner::elide_fns(cctx.instructions);
        cctx.instructions = if_cleaner::optimize( cctx.instructions);
        cctx.instructions = loop_break_inline::loop_break_inline( cctx.instructions);
    }
    cctx.instructions = remap_consts::optimize(cctx.instructions);
    let mut counter = Counter::default();
    let mut fors = Vec::new();
    std::fs::write(
        "cvm/build/struct.optimized.mlasm.cbm",
        format!(
            "vec![{}]",
            cctx.instructions
                .iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .unwrap();
    std::fs::write(
        "cvm/build/new.optimized.mlasm.cbm",
        cctx.instructions
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    let asm: Vec<Asm> = cctx
        .instructions
        .into_iter()
        .map(|x| x.to_asm(&mut counter, &mut fors, None))
        .flatten()
        .collect();
    std::fs::write(
        "cvm/build/new.llasm.cbm",
        asm.iter()
            .enumerate()
            .map(|(x, y)| format!("l{} {}", x, y))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    let clean_asm = Asm::clean(asm);
    let out = clean_asm
        .iter()
        .chain(vec![Asm::End].iter())
        .enumerate()
        .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write("cvm/build/new.cbm", &out).unwrap();

    let p = clean_asm_to_exe(&clean_asm);

    std::fs::write("cvm/build/new.cbmexe", &p).unwrap();

    let out = exe_to_clean_asm(p).unwrap()
        .iter()
        .chain(vec![Asm::End].iter())
        .enumerate()
        .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write("cvm/build/new.decomp.cbm", &out).unwrap();
}
