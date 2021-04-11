#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(associated_type_bounds)]
#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(format_args_capture)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod asm;
pub mod cli;
pub mod cvm_exe;
pub mod cvmir;
pub mod error;
pub mod exporter;
pub mod expression;
pub mod function;
pub mod instruction;
pub mod types;
pub mod utils;
pub mod variable;

use asm::Asm;
use cvm_exe::clean_asm_to_exe;
use cvmir::{
    computer, elide_unused_writes, fn_inliner, if_cleaner, loop_break_inline, remap_consts,
    remove_followed_usages, Counter, IrAsm,
};
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
    time::Instant,
};

use pest::Parser;

use crate::{
    cli::{load_config, CompilerConfig, OutputFormat},
    cvm_exe::CVm,
    cvmir::{clear_unreachable, loop_fn_return_opt},
    exporter::{c, python, rust},
};

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
    pub config: CompilerConfig,
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

fn compile_folder(path: &str, file: &str, execute: bool) {
    let instant = Instant::now();
    let config = load_config(path);
    let build_folder = Path::new(path).join(Path::new(&config.output_folder));
    if !build_folder.exists() {
        std::fs::create_dir_all(&build_folder).unwrap();
    }

    let mut context = CompilationContext::default();

    match compile_file(
        Path::new(path).join(Path::new(file)).to_str().unwrap(),
        &mut context,
    ) {
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
        config: config.compiler,
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

    cctx.instructions.push(IrAsm::End);

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
        if config.optimizer.elide_unused_writes {
            cctx.instructions = elide_unused_writes(cctx.instructions);
        }
        if config.optimizer.remove_followed_usages {
            cctx.instructions = remove_followed_usages(cctx.instructions, None);
        }
        if config.optimizer.regroup_consts {
            cctx.instructions = cvmir::regroup_consts::optimize(cctx.instructions);
        }
        if config.optimizer.compile_time_evaluation {
            cctx.instructions = computer::optimize(cctx.instructions, &config.optimizer);
        }
        if config.optimizer.function_inliner {
            cctx.instructions = fn_inliner::elide_fns(cctx.instructions);
        }
        if config.optimizer.if_optimizer {
            cctx.instructions = if_cleaner::optimize(cctx.instructions);
        }
        if config.optimizer.loop_break_inline {
            cctx.instructions = loop_break_inline::loop_break_inline(cctx.instructions);
        }
        if config.optimizer.loop_fn_return {
            cctx.instructions = loop_fn_return_opt::optimize(cctx.instructions);
        }
        if config.optimizer.clear_unreachable {
            cctx.instructions = clear_unreachable::optimize(cctx.instructions);
        }
    }
    if config.optimizer.remap_consts {
        cctx.instructions = remap_consts::optimize(cctx.instructions);
    }

    if config.output_format.contains(&OutputFormat::Mir) {
        std::fs::write(
            Path::new(path).join(build_folder.join(Path::new("output.mircvm"))),
            cctx.instructions
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
    }
    if config.output_format.contains(&OutputFormat::Python) {
        std::fs::write(
            Path::new(path).join(build_folder.join(Path::new("output.mir.py"))),
            python::export_from_mir(&cctx.instructions),
        )
        .unwrap();
    }
    if config.output_format.contains(&OutputFormat::C) {
        std::fs::write(
            Path::new(path).join(build_folder.join(Path::new("output.mir.c"))),
            c::export_from_mir(&cctx.instructions),
        )
        .unwrap();
    }
    if config.output_format.contains(&OutputFormat::Rust) {
        std::fs::write(
            Path::new(path).join(build_folder.join(Path::new("output_mir.rs"))),
            rust::export_from_mir(&cctx.instructions),
        )
        .unwrap();
    }
    let mut counter = Counter::default();
    let mut fors = Vec::new();
    let asm: Vec<Asm> = cctx
        .instructions
        .into_iter()
        .map(|x| x.to_asm(&mut counter, &mut fors, None))
        .flatten()
        .collect();
    if config.output_format.contains(&OutputFormat::Lir) {
        std::fs::write(
            Path::new(path).join(build_folder.join(Path::new("output.lircvm"))),
            asm.iter()
                .enumerate()
                .map(|(x, y)| format!("l{} {}", x, y))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
    }
    let clean_asm = Asm::clean(asm);
    if config.output_format.contains(&OutputFormat::Asm) {
        let out = clean_asm
            .iter()
            .chain(vec![Asm::End].iter())
            .enumerate()
            .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(
            Path::new(path).join(build_folder.join(Path::new("output.asmcvm"))),
            &out,
        )
        .unwrap();
    }
    if config.output_format.contains(&OutputFormat::Binary) {
        let p = clean_asm_to_exe(&clean_asm);
        std::fs::write(build_folder.join(Path::new("output.execvm")), &p).unwrap();
    }
    println!("Compiled in {:?}", instant.elapsed());
    if execute {
        println!("Executing...");
        let instant = Instant::now();

        let mut cvm_exec = CVm::from_instructions(clean_asm);
        let mut ops: u32 = 0;
        loop {
            ops += 1;
            match cvm_exec.execute() {
                Ok(true) => (),
                Ok(false) => break,
                Err(e) => {
                    println!("Runtime Error : {}", e);
                    break;
                }
            }
        }

        let el = instant.elapsed();

        let ops = match ops {
            1_000_000..=u32::MAX => {
                format!("{:.2}Mops", (ops as f64 / 1_000_000.))
            }
            1_000..1_000_000 => {
                format!("{:.2}Kops", (ops as f64 / 1_000.))
            }
            0..1_000 => {
                format!("{}ops", ops)
            }
        };

        println!("Executed in {:?} ({})", el, ops);
    }
}

fn main() {
    let first_arg = std::env::args().nth(1).unwrap();

    let (first_arg, execute) = if first_arg == "run" {
        (std::env::args().nth(2).unwrap(), true)
    } else {
        (first_arg, false)
    };

    compile_folder(
        std::env::current_dir().unwrap().to_str().unwrap(),
        &first_arg,
        execute,
    );
    // let mut context = CompilationContext::default();
    // match compile_file("cvm/src/game.cvm", &mut context) {
    //     Ok(_) => (),
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // };

    // let func = context
    //     .functions
    //     .get("main")
    //     .unwrap()
    //     .get_for_input(&Vec::new(), &context)
    //     .unwrap()
    //     .clone();
    // let mut cctx = CVMCompCtx {
    //     ctx: context,
    //     current_var: 0,
    //     function: 0,
    //     loops: Vec::new(),
    //     loop_id: 0,
    //     if_n: 0,
    //     instructions: Vec::new(),
    // };
    // match func.compile(&mut cctx, &Vec::new(), None) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // };
    // std::fs::write(
    //     "cvm/build/new.mlasm.cbm",
    //     cctx.instructions
    //         .iter()
    //         .map(|x| format!("{}", x))
    //         .collect::<Vec<_>>()
    //         .join("\n"),
    // )
    // .unwrap();
    // let mut counter = Counter::default();
    // let mut fors = Vec::new();
    // std::fs::write(
    //     "cvm/build/struct.optimized.mlasm.cbm",
    //     format!(
    //         "vec![{}]",
    //         cctx.instructions
    //             .iter()
    //             .map(|x| format!("{:?}", x))
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     ),
    // )
    // .unwrap();
    // std::fs::write(
    //     "cvm/build/new.optimized.mlasm.cbm",
    //     cctx.instructions
    //         .iter()
    //         .map(|x| format!("{}", x))
    //         .collect::<Vec<_>>()
    //         .join("\n"),
    // )
    // .unwrap();
    // let asm: Vec<Asm> = cctx
    //     .instructions
    //     .into_iter()
    //     .map(|x| x.to_asm(&mut counter, &mut fors, None))
    //     .flatten()
    //     .collect();
    // std::fs::write(
    //     "cvm/build/new.llasm.cbm",
    //     asm.iter()
    //         .enumerate()
    //         .map(|(x, y)| format!("l{} {}", x, y))
    //         .collect::<Vec<_>>()
    //         .join("\n"),
    // )
    // .unwrap();
    // let clean_asm = Asm::clean(asm);
    // let out = clean_asm
    //     .iter()
    //     .chain(vec![Asm::End].iter())
    //     .enumerate()
    //     .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // std::fs::write("cvm/build/new.cbm", &out).unwrap();

    // let p = clean_asm_to_exe(&clean_asm);

    // std::fs::write("cvm/build/new.cbmexe", &p).unwrap();

    // let out = exe_to_clean_asm(p)
    //     .unwrap()
    //     .iter()
    //     .chain(vec![Asm::End].iter())
    //     .enumerate()
    //     .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
    //     .collect::<Vec<_>>()
    //     .join("\n");

    // std::fs::write("cvm/build/new.decomp.cbm", &out).unwrap();
}
