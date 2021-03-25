use super::{IrAsm, fn_inliner};

pub fn loop_break_inline(instructions: Vec<IrAsm>) -> Vec<IrAsm>{
    let mut out = Vec::with_capacity(instructions.len());
    let mut iter = instructions.into_iter();
    while let Some(e) = iter.next() {
        if let IrAsm::Loop(e) = e {
            let mut p: Vec<IrAsm> = iter.collect();
            if !fn_inliner::will_return(&p) {
                p.push(IrAsm::Break());
            }
            // println!("A");
            // for i in &e {
            //     if !matches!(i, &IrAsm::FunctionBlock(..)) {
            //         println!("{}",i);
            //     }
            // }
            out.push(IrAsm::Loop(loop_break_inline(replace_inner_breaks(e, &loop_break_inline(p)))));
            return out;
        } else {
            out.push(match e {
                IrAsm::If(a, b, c, d) => {
                    IrAsm::If(a, b, loop_break_inline(c), loop_break_inline(d))
                }
                IrAsm::FunctionBlock(a, b) => {
                    IrAsm::FunctionBlock(a, loop_break_inline(b))
                }
                e => e
            });
        }
    }
    out
}

fn replace_inner_breaks(instructions: Vec<IrAsm>, replace_with: &Vec<IrAsm>) -> Vec<IrAsm> {
    instructions.into_iter().map(|x| {
        vec![match x {
            IrAsm::If(a,b,c,d) => IrAsm::If(a,b,replace_inner_breaks(c,replace_with),replace_inner_breaks(d,replace_with)),
            IrAsm::Break() => {
                println!("REPLACED");
                return replace_with.clone();
            },
            e => e
        }]
    }).flatten().collect()
}