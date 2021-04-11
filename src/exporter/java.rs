use std::collections::HashSet;

use crate::cvmir::IrAsm;

use crate::asm::OperationType;

pub fn export_from_mir(mut mir_code: &[IrAsm]) -> String {
    let mut vars = HashSet::new();
    mir_code.iter().for_each(|x| get_vars(x, &mut vars));
    let defs = vars
        .iter()
        .map(|x| format!("        final Reg v{x} = new Reg();\n"))
        .collect::<Vec<String>>()
        .join("");

    if let Some(&IrAsm::End) = mir_code.last() {
        mir_code = &mir_code[0..mir_code.len() - 1];
    }

    let content = mir_code
        .iter()
        .map(|x| build_instr(x))
        .flatten()
        .map(|x| format!("        {x}\n"))
        .collect::<Vec<_>>()
        .join("");
    include_str!("java_std.java").replace("//%%CODE%%", &format!("{}\n{}", defs, content))
}

pub fn build_instr(asm: &IrAsm) -> Vec<String> {
    vec![match asm {
        IrAsm::Op(a, b, c, d) => {
            let op = match a {
                OperationType::Merge => return vec![format!("v{b}.merge(v{c}, v{d});")],
                e => e.as_operator(),
            };
            format!("v{b}.op(v{c}, v{d}, (a, b) -> a {op} b);")
        }
        IrAsm::Meta(_) => return vec![],
        IrAsm::End => "System.exit(0);".to_owned(),
        IrAsm::If(a, b, c, d) => {
            let mut vec = Vec::new();
            if c.is_empty() {
                vec.push(format!("if (!v{a}.equals(v{b})) {{"));
                vec.extend(
                    d.iter()
                        .map(|x| build_instr(x))
                        .flatten()
                        .map(|x| format!("    {x}")),
                );
            } else {
                vec.push(format!("if (v{a}.equals(v{b})) {{"));
                vec.extend(
                    c.iter()
                        .map(|x| build_instr(x))
                        .flatten()
                        .map(|x| format!("    {x}")),
                );
                if !d.is_empty() {
                    vec.push("} else {".to_owned());
                    vec.extend(
                        d.iter()
                            .map(|x| build_instr(x))
                            .flatten()
                            .map(|x| format!("    {x}")),
                    );
                }
            }
            vec.push("}".to_owned());
            return vec;
        }
        IrAsm::Loop(b) => {
            let mut vec = Vec::new();
            vec.push("while (true) {".to_owned());
            vec.extend(
                b.iter()
                    .map(|x| build_instr(x))
                    .flatten()
                    .map(|x| format!("    {x}")),
            );
            vec.push("}".to_owned());
            return vec;
        }
        IrAsm::Break() => "break;".to_owned(),
        IrAsm::Continue() => "continue;".to_owned(),
        IrAsm::FunctionBlock(a, b) => {
            let mut vec = Vec::new();
            vec.push(format!("v{a}.mov(((Func) () -> {{"));
            vec.extend(
                b.iter()
                    .map(|x| build_instr(x))
                    .flatten()
                    .map(|x| format!("    {x}")),
            );
            vec.push("}).run());".to_owned());
            return vec;
        }
        IrAsm::Return(a) => format!("return v{a};"),
        IrAsm::Prt(a) => format!("v{a}.print();"),
        IrAsm::Inp(a) => format!("v{a}.input();"),
        IrAsm::Cst(a, b) => format!(
            "v{a}.cst({});",
            b.iter().map(u8::to_string).collect::<Vec<_>>().join(", ")
        ),
        IrAsm::Mov(a, b) => format!("v{a}.mov(v{b});"),
        IrAsm::Len(a, b) => format!("v{a}.len(v{b});"),
        IrAsm::Read(a, b, c, d) => {
            format!("v{a}.read(v{b}, v{c}, v{d});")
        }
        IrAsm::Nop => return vec![],
    }]
}

pub fn get_vars(asm: &IrAsm, vars: &mut HashSet<usize>) {
    match asm {
        IrAsm::Op(_, b, c, d) => {
            vars.insert(*b);
            vars.insert(*c);
            vars.insert(*d);
        }
        IrAsm::Meta(_) => {}
        IrAsm::End => {}
        IrAsm::If(b, c, d, e) => {
            vars.insert(*b);
            vars.insert(*c);
            d.iter().for_each(|x| get_vars(x, vars));
            e.iter().for_each(|x| get_vars(x, vars));
        }
        IrAsm::Loop(d) => {
            d.iter().for_each(|x| get_vars(x, vars));
        }
        IrAsm::Break() => {}
        IrAsm::Continue() => {}
        IrAsm::FunctionBlock(c, d) => {
            vars.insert(*c);
            d.iter().for_each(|x| get_vars(x, vars));
        }
        IrAsm::Return(c) | IrAsm::Prt(c) | IrAsm::Inp(c) | IrAsm::Cst(c, _) => {
            vars.insert(*c);
        }
        IrAsm::Mov(c, d) | IrAsm::Len(c, d) => {
            vars.insert(*c);
            vars.insert(*d);
        }
        IrAsm::Read(a, b, c, d) => {
            vars.insert(*a);
            vars.insert(*b);
            vars.insert(*c);
            vars.insert(*d);
        }
        IrAsm::Nop => {}
    }
}
