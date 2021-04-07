use super::IrAsm;

pub fn optimize(ir: Vec<IrAsm>) -> Vec<IrAsm> {
    ir.into_iter().map(|x| optimize_loop(x)).flatten().collect()
}

fn optimize_loop(ir: IrAsm) -> Vec<IrAsm> {
    match ir {
        IrAsm::If(a, b, c, d) => {
            vec![IrAsm::If(
                a,
                b,
                c.into_iter().map(|x| optimize_loop(x)).flatten().collect(),
                d.into_iter().map(|x| optimize_loop(x)).flatten().collect(),
            )]
        }
        IrAsm::Loop(a) => {
            vec![IrAsm::Loop(
                a.into_iter().map(|x| optimize_loop(x)).flatten().collect(),
            )]
        }
        IrAsm::FunctionBlock(a, b) => {
            if !matches!(b.last(), Some(IrAsm::Loop(_))) {
                return vec![IrAsm::FunctionBlock(
                    a,
                    b.into_iter().map(|x| optimize_loop(x)).flatten().collect(),
                )];
            }
            if contains_return(&b[0..(b.len() - 1)]) {
                return vec![IrAsm::FunctionBlock(
                    a,
                    b.into_iter().map(|x| optimize_loop(x)).flatten().collect(),
                )];
            }
            let mut i = b.into_iter().rev();
            let loop_asm_i = if let Some(IrAsm::Loop(e)) = i.next() {
                e
            } else {
                unreachable!()
            };
            let mut rest: Vec<IrAsm> = i.rev().collect();
            rest.push(IrAsm::Loop(replace_returns(a, loop_asm_i)));
            rest
        }
        e => vec![e],
    }
}

fn replace_returns(into: usize, vec: Vec<IrAsm>) -> Vec<IrAsm> {
    vec.into_iter()
        .map(|x| match x {
            IrAsm::If(a, b, c, d) => {
                vec![IrAsm::If(
                    a,
                    b,
                    replace_returns(into, c),
                    replace_returns(into, d),
                )]
            }
            IrAsm::Loop(e) => vec![IrAsm::Loop(replace_returns(into, e))],
            IrAsm::Return(e) => {
                vec![IrAsm::Mov(into, e), IrAsm::Break()]
            }
            e => vec![e],
        })
        .flatten()
        .collect()
}

fn contains_return(vec: &[IrAsm]) -> bool {
    vec.iter().any(|x| match x {
        IrAsm::If(_, _, a, b) => contains_return(a) || contains_return(b),
        IrAsm::Loop(a) => contains_return(a),
        IrAsm::Return(_) => true,
        _ => false,
    })
}
