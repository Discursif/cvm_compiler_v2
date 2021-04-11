use std::collections::HashSet;

use crate::cvmir::IrAsm;

use crate::asm::OperationType;

pub fn export_from_mir(mir_code: &[IrAsm]) -> String {
    let mut vars = HashSet::new();
    mir_code.iter().for_each(|x| get_vars(x, &mut vars));
    let stack = format!(
        "use std::io::BufRead;\nuse std::io::Write;\n\n#[derive(Default)]\nstruct Stack {{\n{}}}\n",
        vars.iter()
            .map(|x| format!("    v{x}: Vec<u8>,\n"))
            .collect::<Vec<String>>()
            .join("")
    );

    let mut counter = 0;
    let main_fn = format!(
        "\nfn main() {{\n    let mut stack = &mut Stack::default();\n{}\n}}\n",
        mir_code
            .iter()
            .map(|x| build_instr(x, &mut counter))
            .flatten()
            .map(|x| format!("    {x}\n"))
            .collect::<Vec<_>>()
            .join("")
    );
    format!("{}{}", stack, main_fn)
}

pub fn build_instr(asm: &IrAsm, fn_counter: &mut usize) -> Vec<String> {
    vec![match asm {
        IrAsm::Op(a, b, c, d) => {
            let op = match a {
                OperationType::Add => "(*a).wrapping_add(*b)",
                OperationType::And => "a & b",
                OperationType::Sub => "a - b",
                OperationType::Mul => "(*a).wrapping_mul(*b)",
                OperationType::Div => "(*a).wrapping_div(*b)",
                OperationType::Mod => "a % b",
                OperationType::Xor => "a ^ b",
                OperationType::Or => "a | b",
                OperationType::Merge => {
                    return vec![format!(
                    "stack.v{b} = stack.v{c}.iter().chain(stack.v{d}.iter()).copied().collect();"
                )]
                }
            };
            format!("stack.v{b} = stack.v{c}.iter().zip(stack.v{d}.iter().cycle()).map(|(a,b)| {op}).collect();")
        }
        IrAsm::Meta(_) => return vec![],
        IrAsm::End => "std::process::exit(0);".to_owned(),
        IrAsm::If(a, b, c, d) => {
            let mut vec = Vec::new();
            if c.is_empty() {
                vec.push(format!("if stack.v{a} != stack.v{b} {{"));
                vec.extend(
                    d.iter()
                        .map(|x| build_instr(x, fn_counter))
                        .flatten()
                        .map(|x| format!("    {x}")),
                );
            } else {
                vec.push(format!("if stack.v{a} == stack.v{b} {{"));
                vec.extend(
                    c.iter()
                        .map(|x| build_instr(x, fn_counter))
                        .flatten()
                        .map(|x| format!("    {x}")),
                );
                if !d.is_empty() {
                    vec.push("} else {".to_owned());
                    vec.extend(
                        d.iter()
                            .map(|x| build_instr(x, fn_counter))
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
            vec.push("loop {".to_owned());
            vec.extend(
                b.iter()
                    .map(|x| build_instr(x, fn_counter))
                    .flatten()
                    .map(|x| format!("    {x}")),
            );
            vec.push("}".to_owned());
            return vec;
        }
        IrAsm::Break() => "break;".to_owned(),
        IrAsm::Continue() => "continue;".to_owned(),
        IrAsm::FunctionBlock(a, b) => {
            *fn_counter += 1;
            let curr = *fn_counter;

            let mut vec = Vec::new();
            vec.push(format!("fn f{curr}(stack: &mut Stack) -> Vec<u8> {{"));
            vec.extend(
                b.iter()
                    .map(|x| build_instr(x, fn_counter))
                    .flatten()
                    .map(|x| format!("    {x}")),
            );
            vec.push(format!("}}"));
            vec.push(format!("stack.v{a} = f{curr}(stack);"));
            return vec;
        }
        IrAsm::Return(a) => format!("return stack.v{a}.clone();"),
        IrAsm::Prt(a) => {
            return vec![
                format!("std::io::stdout().lock().write(&stack.v{a}).unwrap();"),
                "std::io::stdout().lock().flush().unwrap();".to_owned(),
            ]
        }
        IrAsm::Inp(a) => {
            return vec![
                format!("stack.v{a} = Vec::with_capacity(255);"),
                format!("std::io::stdin().lock().read_until(10, &mut stack.v{a}).unwrap();"),
                format!("while let Some(13) | Some(10) = stack.v{a}.last() {{"),
                format!("    stack.v{a}.pop();"),
                "}".to_owned(),
            ]
        }
        IrAsm::Cst(a, b) => format!(
            "stack.v{a} = vec![{}];",
            b.iter().map(u8::to_string).collect::<Vec<_>>().join(", ")
        ),
        IrAsm::Mov(a, b) => format!("stack.v{a} = stack.v{b}.clone();"),
        IrAsm::Len(a, b) => format!("stack.v{a} = vec![stack.v{b}.len() as u8];"),
        IrAsm::Read(a, b, c, d) => {
            format!("stack.v{a} = stack.v{b}.iter().skip(stack.v{c}[0] as usize).take(stack.v{d}[0] as usize).copied().collect();")
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
