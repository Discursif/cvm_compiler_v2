use super::IrAsm;


pub fn elide_fns(ir: Vec<IrAsm>) -> Vec<IrAsm> {
    ir.into_iter().map(|x| {
        vec![match x {
            IrAsm::If(a, b, c, d) => IrAsm::If(a, b, elide_fns(c), elide_fns(d)),
            IrAsm::Loop(e) => IrAsm::Loop(elide_fns(e)),
            IrAsm::FunctionBlock(a, block) => {
                IrAsm::FunctionBlock(a, if does_end_flat(&block) {
                    return elide_fns(replace_returns_by_mov(block.clone(), a));
                } else {
                    elide_fns(block)
                })
            }
            e => e
        }]
    }).flatten().collect()
}

fn replace_returns_by_mov(instruction: Vec<IrAsm>, to: usize) -> Vec<IrAsm> {
    let mut out = Vec::with_capacity(instruction.len());
    for i in instruction {
        match i {
            IrAsm::Continue() => {
                out.push(IrAsm::Continue());
                return out;
            }
            IrAsm::Break() => {
                out.push(IrAsm::Break());
                return out;
            }
            IrAsm::End => {
                out.push(IrAsm::End);
                return out;
            }
            IrAsm::If(a, b, c, d) => {
                if does_end_in_any_case(&c) && does_end_in_any_case(&d) {
                    out.push(IrAsm::If(
                        a,
                        b,
                        replace_returns_by_mov(c, to),
                        replace_returns_by_mov(d, to),
                    ));
                    return out;
                }
                out.push(IrAsm::If(
                    a,
                    b,
                    replace_returns_by_mov(c, to),
                    replace_returns_by_mov(d, to),
                ));
            }
            IrAsm::Loop(e) => {
                if does_end_in_any_case(&e) {
                    out.push(IrAsm::Loop(replace_returns_by_mov(e, to)));
                    return out;
                }
                out.push(IrAsm::Loop(replace_returns_by_mov(e, to)));
            }
            IrAsm::Return(a) => {
                out.push(IrAsm::Mov(to, a));
                return out;
            }
            e => out.push(e),
        }
    }
    out
}

fn inner(instruction: &Vec<IrAsm>, scope: bool) -> bool {
    for i in instruction {
        match i {
            IrAsm::End => return true,
            IrAsm::If(_, _, a, b) => {
                let a = inner(a, false);
                let b = inner(b, false);
                if a && b {
                    return true;
                } else if a || b {
                    return false;
                }
            }
            IrAsm::Loop(a) => {
                if can_return(a) {
                    return false;
                }
            }
            IrAsm::Break() => return scope,
            IrAsm::Continue() => return scope,
            IrAsm::Return(_) => return true,
            _ => (),
        }
    }
    return false;
}

fn does_end_flat(instruction: &Vec<IrAsm>) -> bool {
    !can_return(instruction) || inner(instruction, true)
}

pub fn does_end_in_any_case(instruction: &Vec<IrAsm>) -> bool {
    inner(instruction, true)
}

pub fn will_return(instruction: &Vec<IrAsm>) -> bool {
    for i in instruction {
        match i {
            IrAsm::End => return true,
            IrAsm::If(_, _, a, b) => {
                let a = inner(a, false);
                let b = inner(b, false);
                if a && b {
                    return true;
                }
            }
            IrAsm::Loop(a) => {
                if can_return(a) {
                    return false;
                }
            }
            IrAsm::Break() => return true,
            IrAsm::Continue() => return true,
            IrAsm::Return(_) => return true,
            _ => (),
        }
    }
    return false;
}

fn can_return(instruction: &Vec<IrAsm>) -> bool {
    for i in instruction {
        match i {
            IrAsm::If(_, _, a, b) => {
                if can_return(a) || can_return(b) {
                    return true;
                }
            }
            IrAsm::Loop(e) => {
                if can_return(e) {
                    return true;
                }
            }
            IrAsm::Return(_) => return true,
            _ => {}
        }
    }
    false
}