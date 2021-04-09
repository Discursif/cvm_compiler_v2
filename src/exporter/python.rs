use crate::cvmir::IrAsm;

pub fn export_from_mir(mir_code: &[IrAsm]) -> String {
    let file_base = include_str!("python_std.py");
    let mut trs = TranspileContext::default();
    format!(
        "{file_base}{}",
        mir_code
            .iter()
            .map(|x| transpile(x, &mut trs))
            .flatten()
            .collect::<Vec<_>>()
            .join("\n")
    )
}

use crate::asm::OperationType;

// def tmp1(a):
//   return a
// out = tmp1()

#[derive(Default)]
struct TranspileContext {
    fn_number: usize,
}

fn transpile(mir_code: &IrAsm, context: &mut TranspileContext) -> Vec<String> {
    vec![match mir_code {
        IrAsm::Op(operator, into, a, b) => {
            let operator = match operator {
                OperationType::Merge => {
                    return vec![format!("v[{into}]=v[{a}]+v[{b}]")];
                }
                OperationType::Div => {
                    return vec![format!("v[{into}]=list(map(lambda i:floor(v[{a}][i]/v[{b}][i%len(v[{b}])])%256,iter(range(len(v[{a}])))))")];
                }
                e => e.as_operator(),
            };
            format!("v[{into}]=list(map(lambda i:(v[{a}][i]{operator}v[{b}][i%len(v[{b}])])%256,iter(range(len(v[{a}])))))")
        }
        IrAsm::End => "exit()".to_owned(),
        IrAsm::If(a, b, c, d) => {
            if c.is_empty() {
                let mut vec = Vec::new();
                vec.push(format!("if v[{a}]!=v[{b}]:"));
                vec.extend(
                    d.iter()
                        .map(|x| transpile(x, context))
                        .flatten()
                        .map(|x| format!("  {}", x)),
                );
                return vec;
            } else {
                let mut vec = Vec::new();
                vec.push(format!("if v[{a}]==v[{b}]:"));
                vec.extend(
                    c.iter()
                        .map(|x| transpile(x, context))
                        .flatten()
                        .map(|x| format!("  {}", x)),
                );
                if !d.is_empty() {
                    vec.push(format!("else:"));
                    vec.extend(
                        d.iter()
                            .map(|x| transpile(x, context))
                            .flatten()
                            .map(|x| format!("  {}", x)),
                    );
                }
                return vec;
            }
        }
        IrAsm::Loop(e) => {
            let mut vec = Vec::new();
            vec.push(format!("while True:"));
            vec.extend(
                e.iter()
                    .map(|x| transpile(x, context))
                    .flatten()
                    .map(|x| format!("  {}", x)),
            );
            return vec;
        }
        IrAsm::Break() => "break".to_owned(),
        IrAsm::Continue() => "continue".to_owned(),
        IrAsm::FunctionBlock(a, b) => {
            let mut vec = Vec::new();
            vec.push(format!("def f{}():", context.fn_number));
            let curr = context.fn_number;
            context.fn_number += 1;
            vec.extend(
                b.iter()
                    .map(|x| transpile(x, context))
                    .flatten()
                    .map(|x| format!("  {}", x)),
            );
            vec.push(format!("v[{a}]=f{}()", curr));
            return vec;
        }
        IrAsm::Return(a) => format!("return v[{a}]"),
        IrAsm::Prt(a) => format!("p(v[{a}])"),
        IrAsm::Inp(a) => format!("v[{a}]=list(map(lambda a:ord(a),iter(input())))"),
        IrAsm::Cst(a, b) => format!("v[{a}]={:?}", b).replace(" ", ""),
        IrAsm::Mov(a, b) => format!("v[{a}]=v[{b}]"),
        IrAsm::Len(a, b) => format!("v[{a}]=[len(v[{b}])]"),
        IrAsm::Read(a, b, c, d) => {
            format!("v[{a}]=v[{b}][v[{c}][0]:v[{c}][0]+v[{d}][0]]")
        }
        IrAsm::Nop => return vec![],
    }]
}
