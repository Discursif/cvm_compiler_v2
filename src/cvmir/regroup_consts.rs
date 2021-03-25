use std::collections::{HashMap, HashSet};

use super::IrAsm;

pub struct Vars {
    pub consts: HashMap<usize, Vec<u8>>,
    unusable: HashSet<usize>,
    merged: HashMap<Vec<u8>, Vec<usize>>,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            consts: HashMap::new(),
            merged: HashMap::new(),
            unusable: HashSet::new(),
        }
    }
    pub fn build(&self) -> Vec<IrAsm> {
        self.merged
            .iter()
            .map(|x| IrAsm::Cst(x.1[0], x.0.clone()))
            .collect()
    }
    pub fn update(&mut self) {
        for (x, y) in &self.consts {
            if let Some(e) = self.merged.get_mut(y) {
                e.push(*x);
            } else {
                self.merged.insert(y.clone(), vec![*x]);
            }
        }
    }
    pub fn get(&mut self, a: usize) {
        self.consts.remove(&a);
        self.unusable.insert(a);
    }

    pub fn add(&mut self, a: usize, b: Vec<u8>) {
        self.consts.remove(&a);
        if !self.unusable.contains(&a) {
            self.consts.insert(a, b.clone());
        }
        self.unusable.insert(a);
    }

    pub fn remap(&self, input: usize) -> usize {
        if let Some(e) = self.merged.iter().find_map(|(_, y)| {
            if y.iter().any(|u| u == &input) {
                Some(y[0])
            } else {
                None
            }
        }) {
            e
        } else {
            input
        }
    }

    pub fn need_to_be_deleted(&self, input: usize) -> bool {
        self.merged
            .iter()
            .any(|(_, y)| y.iter().any(|x| x == &input))
    }
}

fn inner_opt(asm: Vec<IrAsm>, consts: &mut Vars) -> Vec<IrAsm> {
    asm.into_iter()
        .flat_map(|x| {
            Some(match x {
                IrAsm::Op(a, b, c, d) => IrAsm::Op(a, b, consts.remap(c), consts.remap(d)),
                IrAsm::End => IrAsm::End,
                IrAsm::If(a, b, c, d) => IrAsm::If(
                    consts.remap(a),
                    consts.remap(b),
                    inner_opt(c, &mut *consts),
                    inner_opt(d, &mut *consts),
                ),
                IrAsm::Loop(a) => IrAsm::Loop(inner_opt(a, &mut *consts)),
                IrAsm::Break() => IrAsm::Break(),
                IrAsm::Continue() => IrAsm::Continue(),
                IrAsm::FunctionBlock(a, b) => IrAsm::FunctionBlock(a, inner_opt(b, &mut *consts)),
                IrAsm::Return(a) => IrAsm::Return(consts.remap(a)),
                IrAsm::Prt(a) => IrAsm::Prt(consts.remap(a)),
                IrAsm::Inp(a) => IrAsm::Inp(a),
                IrAsm::Cst(a, b) => {
                    if consts.need_to_be_deleted(a) {
                        return None;
                    }
                    IrAsm::Cst(a, b)
                }
                IrAsm::Mov(a, b) => IrAsm::Mov(a, consts.remap(b)),
                IrAsm::Len(a, b) => IrAsm::Len(a, consts.remap(b)),
                IrAsm::Read(a, b, c, d) => {
                    IrAsm::Read(a, consts.remap(b), consts.remap(c), consts.remap(d))
                }
                IrAsm::Nop => IrAsm::Nop,
            })
        })
        .collect()
}

pub fn optimize(asm: Vec<IrAsm>) -> Vec<IrAsm> {
    let mut consts = Vars::new();
    asm.iter().for_each(|x| get_writes(x, &mut consts));
    consts.update();
    let mut vec = consts.build();
    vec.extend(inner_opt(asm, &mut consts).into_iter());
    vec
}

pub fn get_writes(asm: &IrAsm, consts: &mut Vars) {
    match asm {
        IrAsm::Op(_, b, _, _) => {
            consts.get(*b);
        }
        IrAsm::End => {}
        IrAsm::If(_, _, d, e) => {
            d.iter().for_each(|x| get_writes(x, consts));
            e.iter().for_each(|x| get_writes(x, consts));
        }
        IrAsm::Loop(e) => {
            e.iter().for_each(|x| get_writes(x, consts));
        }
        IrAsm::Break() => {}
        IrAsm::Continue() => {}
        IrAsm::FunctionBlock(a, e) => {
            consts.get(*a);
            e.iter().for_each(|x| get_writes(x, consts));
        }
        IrAsm::Return(_) => {}
        IrAsm::Prt(_) => {}
        IrAsm::Inp(a) => {
            consts.get(*a);
        }
        IrAsm::Cst(a, b) => {
            consts.add(*a, b.clone());
        }
        IrAsm::Mov(a, _) => {
            consts.get(*a);
        }
        IrAsm::Len(a, _) => {
            consts.get(*a);
        }
        IrAsm::Read(a, _, _, _) => {
            consts.get(*a);
        }
        IrAsm::Nop => {}
    }
}
