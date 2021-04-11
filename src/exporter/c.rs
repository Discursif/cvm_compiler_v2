use std::collections::HashSet;

use crate::cvmir::IrAsm;

use crate::asm::OperationType;

impl IrAsm {
    fn to_c_string(
        &self,
        return_reg: Option<usize>,
        fn_id: usize,
        fn_id_alloc: &mut usize,
    ) -> String {
        match self {
            IrAsm::Op(o, d, o1, o2) => {
                match o {
                    OperationType::Merge => {
                        format!(r#"if (v{o1}[0] + v{o2}[0] > 255) abort();
                                    v{d}[0] = v{o1}[0] + v{o2}[0];
                                    memcpy(v{d}+1, v{o1}+1, v{o1}[0]);
                                    memcpy(v{d}+v{o1}[0]+1, v{o2}+1, v{o2}[0]);"#)
                    },
                    e => {
                        format!(r#"v{d}[0] = v{o1}[0]; for (unsigned char index = 0; index < v{o1}[0]; index++) {{
                            v{d}[index+1] = v{o1}[index+1] {} v{o2}[(index % v{o2}[0])+1];
                        }}"#,e.as_operator())
                    }
                }
            }, //FIXME
            IrAsm::Meta(_) => String::new(),
            IrAsm::End => "exit(0);".to_owned(),
            IrAsm::If(o1, o2, then, r#else) =>
                format!("if (v{o1}[0] == v{o2}[0] && memcmp(v{o1}+1, v{o2}+1, v{o1}[0]) == 0) {{\n{}\n}} else {{ \n{} \n}}", 
                then.iter().map(|x| x.to_c_string(return_reg,fn_id,fn_id_alloc)).collect::<Vec<_>>().join("\n"),
                r#else.iter().map(|x| x.to_c_string(return_reg,fn_id,fn_id_alloc)).collect::<Vec<_>>().join("\n")
            ),
            IrAsm::Loop(l) =>
                format!("while(1) {{\n{}\n}}",
                l.iter().map(|x| x.to_c_string(return_reg,fn_id,fn_id_alloc)).collect::<Vec<_>>().join("\n")
            ),
            IrAsm::Break() => "break;".to_owned(),
            IrAsm::Continue() => "continue;".to_owned(),
            IrAsm::FunctionBlock(a, b) => {
                *fn_id_alloc += 1;
                let current_id = *fn_id_alloc;
                format!(r#"
                {}
                fn{current_id}:
                "#, b.iter().map(|x| x.to_c_string(Some(*a),current_id,fn_id_alloc)).collect::<Vec<_>>().join("\n"))
            },
            IrAsm::Return(v) => format!("memcpy(v{}, v{v}, v{v}[0]+1); goto fn{fn_id};", return_reg.unwrap()),
            IrAsm::Prt(i) => format!("v{i}[v{i}[0]+1] = 0; printf(\"%s\", v{i}+1);"),
            IrAsm::Inp(o) => format!("fgets(v{o}+1, 255, stdin); v{o}[0] = strlen(v{o}+1)-1;"),
            IrAsm::Cst(o, i) => format!("memcpy(v{o}, (char[{}]){{{}, {}}}, {});", i.len() + 1,i.len(), i.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "),i.len() + 1),
            IrAsm::Mov(d, s) => format!("memcpy(v{d}, v{s}, v{s}[0]+1);"),
            IrAsm::Len(d, s) => format!("v{d}[0] = 1; v{d}[1] = v{s}[0];"),
            IrAsm::Read(d, s, offset, length) =>
                //a = b.iter().skip(c[0]).take(d[0]).collect()
                format!(r#"if (v{offset}[0] != 1 || v{length}[0] != 1 || v{offset}[1] >= v{s}[0]) abort();
                v{d}[0] = MIN(v{s}[0] - v{offset}[1], v{length}[1]);
                memcpy(v{d}+1, v{s}+v{offset}[1]+1, v{d}[0]);"#
            ),
            IrAsm::Nop => String::new(),
        }
    }
}

pub fn export_from_mir(mir_code: &[IrAsm]) -> String {
    let mut vars = HashSet::new();
    mir_code.iter().for_each(|x| get_vars(x, &mut vars));

    let defs = vars
        .iter()
        .map(|x| format!("unsigned char v{}[257];\n", x))
        .collect::<Vec<_>>()
        .join("\n");

    let mut count = 0;
    let stmts = mir_code
        .iter()
        .map(|x| x.to_c_string(None, 0, &mut count))
        .collect::<Vec<_>>()
        .join("\n");

    return include_str!("c_std.c")
        .replace("//%%DEFS%%", &defs)
        .replace("//%%CODE%%", &stmts);
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

/* v<varnumber>_value char[255]
v<varnumber>_len char */
