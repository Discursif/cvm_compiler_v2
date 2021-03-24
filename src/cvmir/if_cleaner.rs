use std::collections::HashMap;

use super::{IrAsm, regroup_consts::{Vars, get_writes}};

pub fn optimize(cvm_ir: Vec<IrAsm>) -> Vec<IrAsm> {
    let mut writes = Vars::new();
    cvm_ir.iter().for_each(|x| get_writes(x, &mut writes));
    optimize_inner(cvm_ir, &writes.consts)
}

fn optimize_inner(cvm_ir: Vec<IrAsm>, consts: &HashMap<usize, Vec<u8>>) -> Vec<IrAsm> {
    let mut iter = cvm_ir.into_iter();
    let mut buffer = Vec::new();
    let mut last = iter.next().map(|e| {
        match e {
            IrAsm::If(a, b, c, d) => {
                IrAsm::If(a, b, optimize_inner(c,consts), optimize_inner(d,consts))
            }
            IrAsm::Loop(a) => {
                IrAsm::Loop(optimize_inner(a,consts))
            }
            IrAsm::FunctionBlock(a, b) => {
                IrAsm::FunctionBlock(a, optimize_inner(b,consts))
            },
            e => e
        }
    });
    while let Some(e) = iter.next() {
        let e = match e {
            IrAsm::If(a, b, c, d) => {
                IrAsm::If(a, b, optimize_inner(c,consts), optimize_inner(d,consts))
            }
            IrAsm::Loop(a) => {
                IrAsm::Loop(optimize_inner(a,consts))
            }
            IrAsm::FunctionBlock(a, b) => {
                IrAsm::FunctionBlock(a, optimize_inner(b,consts))
            },
            e => e
        };
        let other = last.unwrap();
            if let (IrAsm::If(a,b,c,d), IrAsm::If(e,f,g,h)) = (&other,&e) {
                if c.len() == 1 && d.len() == 1 {
                    if let ([IrAsm::Cst(v,w)], [IrAsm::Cst(x,y)]) = (c.as_slice(),d.as_slice()) {
                        if v == x && (w.as_slice() == [1] || y.as_slice() == [1]) {
                            if e == v {
                                if consts.get(&f).map(|f| f.as_slice() == [1]).unwrap_or(false) {
                                    let (b1, b2) = if w.as_slice() == [1] {
                                        (g,h)
                                    } else {
                                        (h,g)
                                    };
                                    last = Some(IrAsm::If(*a,*b,b1.clone(),b2.clone()));
                                    continue;
                                }
                            } else if f == v {
                                if consts.get(&e).map(|e| e.as_slice() == [1]).unwrap_or(false) {
                                    let (b1, b2) = if w.as_slice() == [1] {
                                        (g,h)
                                    } else {
                                        (h,g)
                                    };
                                    last = Some(IrAsm::If(*a,*b,b1.clone(),b2.clone()));
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            last = Some(e);
        buffer.push(other);
    }
    if let Some(e) = last {
        buffer.push(e);
    }
    buffer
}