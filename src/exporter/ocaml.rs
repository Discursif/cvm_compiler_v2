use std::collections::HashSet;

use crate::cvmir::IrAsm;

use crate::asm::{Asm, OperationType};
use crate::instruction::AsmInstruction;

fn to_ocaml_string(instruction: &Asm, line: usize) -> String {
    match instruction {
        Asm::Label(_) => unimplemented!("Should not be called label should have been flattened at this point"),
        Asm::Op(a, b, c, d) => {
            format!("r{b} := reg_update !r{c} !r{d} ({}) 0",match a {
                OperationType::Add => "+",
                OperationType::And => "&",
                OperationType::Sub => "-",
                OperationType::Mul => " * ",
                OperationType::Div => "/",
                OperationType::Mod => "mod",
                OperationType::Xor => "^",
                OperationType::Or => "|",
                OperationType::Merge => return format!("r{b} := !r{c} @ !r{d}"),
            })
        },
        Asm::Gt(a) =>  format!("e {}", a),
        Asm::GtLabel(_) => unimplemented!("Should not be called label should have been flattened at this point"),
        Asm::End => format!("e (-1)"),
        Asm::If(a, b, c) => format!("if r{b} {} r{c} then e {} else e {}", if *a { "<>" } else { "=" },line +2, line +1),
        Asm::Prt(a) => format!("print_string (String.init (List.length !r{a}) (fun n -> (List.nth !r{a} n |> Char.chr)))"),
        Asm::Inp(a) => format!("r{a} := let k = read_line () in init (String.length k) (fun a -> Char.code k.[a])"),
        Asm::Cst(a, b) => format!("r{a} := [{}]",b.iter().map(u8::to_string).collect::<Vec<_>>().join(";")),
        Asm::Mov(a, b) => format!("r{a} := !r{b}"),
        Asm::Len(a, b) => format!("r{a} := [List.length !r{b}]"),
        Asm::Read(a, b, c, d) => format!("r{a} := filteri (fun i _ -> i >= (List.nth !r{c} 0) && i < ((List.nth !r{c} 0) + (List.nth !r{d} 0))) !r{b}"),
        Asm::Nop => format!("()"),
    }
}

pub fn export_from_lir(mir_code: &[Asm]) -> String {
    let mut vars = HashSet::new();
    mir_code.iter().for_each(|x| get_vars(x, &mut vars));

    let defs = vars
        .iter()
        .map(|x| format!("  let r{x} = ref [] in"))
        .collect::<Vec<_>>()
        .join("\n");

    let stmts = mir_code
        .iter()
        .enumerate()
        .map(|(i, x)| format!("    | {i} -> {}", to_ocaml_string(x, i)))
        .collect::<Vec<_>>()
        .join("\n");

    return include_str!("ocaml_std.ml")
        .replace("(*%%DEFS%%*)", &defs)
        .replace("(*%%CODE%%*)", &stmts);
}

fn get_vars(asm: &Asm, vars: &mut HashSet<usize>) {
    match asm {
        Asm::Op(_, b, c, d) => {
            vars.insert(*b);
            vars.insert(*c);
            vars.insert(*d);
        }
        Asm::Label(_) => (),
        Asm::Gt(_) => (),
        Asm::GtLabel(_) => (),
        Asm::End => (),
        Asm::If(_, b, a) => {
            vars.insert(*a);
            vars.insert(*b);
        }
        Asm::Prt(a) => {
            vars.insert(*a);
        }
        Asm::Inp(a) => {
            vars.insert(*a);
        }
        Asm::Cst(a, _) => {
            vars.insert(*a);
        }
        Asm::Mov(a, b) => {
            vars.insert(*a);
            vars.insert(*b);
        }
        Asm::Len(a, b) => {
            vars.insert(*a);
            vars.insert(*b);
        }
        Asm::Read(a, b, c, d) => {
            vars.insert(*a);
            vars.insert(*b);
            vars.insert(*c);
            vars.insert(*d);
        }
        Asm::Nop => (),
    }
}

/* v<varnumber>_value char[255]
v<varnumber>_len char */
