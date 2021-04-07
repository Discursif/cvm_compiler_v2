use crate::cvmir::IrAsm;

pub fn optimize(asm: Vec<IrAsm>) -> Vec<IrAsm> {
    let mut out = Vec::with_capacity(asm.len());
    let mut iter = asm.into_iter();
    while let Some(e) = iter.next() {
        match e {
            IrAsm::If(a, b, c, d) => {
                if finish_in_any_case(&c) && finish_in_any_case(&d) {
                    out.push(IrAsm::If(a, b, optimize(c), optimize(d)));
                    break;
                }
                out.push(IrAsm::If(a, b, optimize(c), optimize(d)));
            }
            IrAsm::FunctionBlock(a, b) => out.push(IrAsm::FunctionBlock(a, optimize(b))),
            IrAsm::Loop(x) => out.push(IrAsm::Loop(optimize(x))),
            IrAsm::Return(a) => {
                out.push(IrAsm::Return(a));
                break;
            }
            IrAsm::Continue() => {
                out.push(IrAsm::Continue());
                break;
            }
            IrAsm::Break() => {
                out.push(IrAsm::Break());
                break;
            }
            IrAsm::End => {
                out.push(IrAsm::End);
                break;
            }
            e => out.push(e),
        }
    }
    out
}

pub fn finish_in_any_case(ir: &[IrAsm]) -> bool {
    for i in ir {
        match i {
            IrAsm::End => return true,
            IrAsm::If(_, _, b, c) => {
                if finish_in_any_case(b) && finish_in_any_case(c) {
                    return true;
                }
            }
            IrAsm::Loop(_) => return true,
            IrAsm::Break() => return true,
            IrAsm::Continue() => return true,
            IrAsm::Return(_) => return true,
            _ => (),
        }
    }
    return false;
}
