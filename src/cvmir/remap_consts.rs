use std::collections::HashMap;

use super::IrAsm;

macro_rules! fix {
    ($i:expr,$e:expr,$c:expr) => {
        get_or_alloc($i,&mut *$e,&mut *$c)
    }
}

pub fn optimize(input: Vec<IrAsm>) -> Vec<IrAsm> {
    fn get_or_alloc(i: usize,consts: &mut HashMap<usize, usize>, counter: &mut usize) -> usize{
        if let Some(e) = consts.get(&i) {
            *e
        } else {
            consts.insert(i,*counter);
            let p = *counter;
            *counter += 1;
            p
        }
    }
    fn inner(input: Vec<IrAsm>, consts: &mut HashMap<usize, usize>, counter: &mut usize) -> Vec<IrAsm> {
        input.into_iter().map(|x| {
            match x {
                IrAsm::Op(a, b, c, d) => IrAsm::Op(a, fix!(b,consts,counter), fix!(c,consts,counter), fix!(d,consts,counter)),
                IrAsm::End => IrAsm::End,
                IrAsm::If(a, b, c, d) => IrAsm::If(fix!(a,consts,counter), fix!(b,consts,counter), inner(c, consts, counter), inner(d, consts, counter)),
                IrAsm::Loop(e) => IrAsm::Loop(inner(e, consts, counter)),
                IrAsm::Break() => IrAsm::Break(),
                IrAsm::Continue() => IrAsm::Continue(),
                IrAsm::FunctionBlock(a, b) => IrAsm::FunctionBlock(fix!(a,consts,counter), inner(b, consts, counter)),
                IrAsm::Return(a) => IrAsm::Return(fix!(a,consts,counter)),
                IrAsm::Prt(a) => IrAsm::Prt(fix!(a,consts,counter)),
                IrAsm::Inp(a) => IrAsm::Inp(fix!(a,consts,counter)),
                IrAsm::Cst(a, b) => IrAsm::Cst(fix!(a,consts,counter),b),
                IrAsm::Mov(a, b) => IrAsm::Mov(fix!(a,consts,counter),fix!(b,consts,counter)),
                IrAsm::Len(a, b) => IrAsm::Len(fix!(a,consts,counter),fix!(b,consts,counter)),
                IrAsm::Read(a, b, c, d) => IrAsm::Read(fix!(a,consts,counter), fix!(b,consts,counter), fix!(c,consts,counter), fix!(d,consts,counter)),
                IrAsm::Nop => IrAsm::Nop
            }
        }).collect()
    }
    inner(input, &mut HashMap::new(), &mut 0)
}