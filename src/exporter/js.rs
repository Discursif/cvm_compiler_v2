use crate::cvmir::IrAsm;

pub fn export_from_mir(mir_code: &[IrAsm]) -> String {
    let file_base = include_str!("js_std.js");
    file_base.replace(
        "//%%CODE%%",
        &mir_code
            .iter()
            .map(|x| transpile(x))
            .flatten()
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

use crate::asm::OperationType;

// def tmp1(a):
//   return a
// out = tmp1()

fn transpile(mir_code: &IrAsm) -> Vec<String> {
    vec![match mir_code {
        IrAsm::Op(operator, into, a, b) => {
            let operator = match operator {
                OperationType::Merge => {
                    return vec![format!("v[{into}]=v[{a}].concat(v[{b}]);")];
                }
                OperationType::Div => {
                    return vec![format!(
                        "v[{into}]=o(v[{a}],v[{b}],(a,b) => Math.floor(a/b));"
                    )];
                }
                e => e.as_operator(),
            };
            format!("v[{into}]=o(v[{a}],v[{b}],(a,b) => a{operator}b);")
        }
        IrAsm::End => "fail;".to_owned(),
        IrAsm::If(a, b, c, d) => {
            if c.is_empty() {
                let mut vec = Vec::new();
                vec.push(format!("if (!arraysMatch(v[{a}],v[{b}])) {{"));
                vec.extend(
                    d.iter()
                        .map(|x| transpile(x))
                        .flatten()
                        .map(|x| format!("  {}", x)),
                );
                vec.push(format!("}}"));
                return vec;
            } else {
                let mut vec = Vec::new();
                vec.push(format!("if (arraysMatch(v[{a}],v[{b}])) {{"));
                vec.extend(
                    c.iter()
                        .map(|x| transpile(x))
                        .flatten()
                        .map(|x| format!("  {}", x)),
                );
                if !d.is_empty() {
                    vec.push(format!("}} else {{"));
                    vec.extend(
                        d.iter()
                            .map(|x| transpile(x))
                            .flatten()
                            .map(|x| format!("  {}", x)),
                    );
                }
                vec.push(format!("}}"));
                return vec;
            }
        }
        IrAsm::Loop(e) => {
            let mut vec = Vec::new();
            vec.push(format!("while (true) {{"));
            vec.extend(
                e.iter()
                    .map(|x| transpile(x))
                    .flatten()
                    .map(|x| format!("  {}", x)),
            );
            vec.push(format!("}}"));
            return vec;
        }
        IrAsm::Break() => "break;".to_owned(),
        IrAsm::Continue() => "continue;".to_owned(),
        IrAsm::FunctionBlock(a, b) => {
            let mut vec = Vec::new();
            vec.push(format!("v[{a}]=(function() {{"));
            vec.extend(
                b.iter()
                    .map(|x| transpile(x))
                    .flatten()
                    .map(|x| format!("  {}", x)),
            );
            vec.push(format!("}})();"));
            return vec;
        }
        IrAsm::Return(a) => format!("return v[{a}]"),
        IrAsm::Prt(a) => format!("print(v[{a}])"),
        IrAsm::Inp(a) => format!("v[{a}]=await input();"),
        IrAsm::Cst(a, b) => format!("v[{a}]={:?}", b).replace(" ", ""),
        IrAsm::Mov(a, b) => format!("v[{a}]=v[{b}];"),
        IrAsm::Len(a, b) => format!("v[{a}]=[v[{b}].length];"),
        IrAsm::Read(a, b, c, d) => {
            format!("v[{a}]=v[{b}].slice(v[{c}][0],v[{c}][0]+v[{d}][0]);")
        }
        IrAsm::Nop => return vec![],
        IrAsm::Meta(_) => return vec![],
    }]
}
