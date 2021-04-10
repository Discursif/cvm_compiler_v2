use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::asm::{Asm, OperationType};

type Variable = usize;

pub mod clear_unreachable;
pub mod computer;
pub mod fn_inliner;
pub mod if_cleaner;
pub mod loop_break_inline;
pub mod loop_fn_return_opt;
pub mod loop_for_unwrapper;
pub mod regroup_consts;
pub mod remap_consts;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrAsm {
    Op(OperationType, Variable, Variable, Variable),
    Meta(IrAsmMeta),
    End,
    If(Variable, Variable, Vec<Self>, Vec<Self>),
    Loop(Vec<Self>),
    Break(),
    Continue(),
    FunctionBlock(Variable, Vec<Self>),
    Return(Variable),
    Prt(Variable),
    Inp(Variable),
    Cst(Variable, Vec<u8>),
    Mov(Variable, Variable),
    Len(Variable, Variable),
    Read(Variable, Variable, Variable, Variable),
    Nop,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IrAsmMeta {
    SetLength(Variable, usize),
}
impl Display for IrAsmMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrAsmMeta::SetLength(a, b) => {
                write!(f, "len({}) = {}", a, b)
            }
        }
    }
}

impl Debug for IrAsm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrAsm::Op(a, b, c, d) => {
                write!(f, "IrAsm::Op(OperationType::{:?}, {}, {}, {})", a, b, c, d)
            }
            IrAsm::End => {
                write!(f, "IrAsm::End")
            }
            IrAsm::If(a, b, c, d) => {
                write!(
                    f,
                    "IrAsm::If({}, {}, vec![{}], vec![{}])",
                    a,
                    b,
                    c.iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<_>>()
                        .join(", "),
                    d.iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            IrAsm::Loop(c) => {
                write!(
                    f,
                    "IrAsm::Loop(vec![{}])",
                    c.iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            IrAsm::Break() => {
                write!(f, "IrAsm::Break()")
            }
            IrAsm::Continue() => {
                write!(f, "IrAsm::Continue()")
            }
            IrAsm::FunctionBlock(a, b) => {
                write!(
                    f,
                    "IrAsm::FunctionBlock({}, vec![{}])",
                    a,
                    b.iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            IrAsm::Return(a) => {
                write!(f, "IrAsm::Return({})", a)
            }
            IrAsm::Prt(a) => {
                write!(f, "IrAsm::Prt({})", a)
            }
            IrAsm::Inp(a) => {
                write!(f, "IrAsm::Inp({})", a)
            }
            IrAsm::Cst(a, b) => {
                write!(
                    f,
                    "IrAsm::Cst({}, vec![{}])",
                    a,
                    b.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            IrAsm::Mov(a, b) => {
                write!(f, "IrAsm::Mov({}, {})", a, b)
            }
            IrAsm::Len(a, b) => {
                write!(f, "IrAsm::Len({}, {})", a, b)
            }
            IrAsm::Read(a, b, c, d) => {
                write!(f, "IrAsm::Read({}, {}, {}, {})", a, b, c, d)
            }
            IrAsm::Nop => {
                write!(f, "IrAsm::NoOp")
            }
            IrAsm::Meta(e) => write!(f, "IrAsm::Meta(IrAsmMeta::{:?})", e),
        }
    }
}

impl Display for IrAsm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrAsm::Op(a, b, c, d) => write!(f, "v{} = v{} {} v{}", b, c, a.as_operator(), d),
            IrAsm::End => write!(f, "end"),
            IrAsm::If(a, b, c, d) => {
                if c.is_empty() {
                    write!(
                        f,
                        "if v{} != v{} {{\n  {}\n}}",
                        a,
                        b,
                        d.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join("\n")
                            .replace("\n", "\n  ")
                    )
                } else if d.is_empty() {
                    write!(
                        f,
                        "if v{} == v{} {{\n  {}\n}}",
                        a,
                        b,
                        c.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join("\n")
                            .replace("\n", "\n  ")
                    )
                } else {
                    write!(
                        f,
                        "if v{} == v{} {{\n  {}\n}} else {{\n  {}\n}}",
                        a,
                        b,
                        c.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join("\n")
                            .replace("\n", "\n  "),
                        d.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join("\n")
                            .replace("\n", "\n  ")
                    )
                }
            }
            IrAsm::Loop(e) => {
                write!(
                    f,
                    "loop {{\n  {}\n}}",
                    e.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                        .replace("\n", "\n  ")
                )
            }
            IrAsm::Break() => write!(f, "break"),
            IrAsm::Continue() => write!(f, "continue"),
            IrAsm::FunctionBlock(e, a) => {
                write!(
                    f,
                    "v{} = fn {{\n  {}\n}}",
                    e,
                    a.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                        .replace("\n", "\n  ")
                )
            }
            IrAsm::Return(e) => write!(f, "return v{}", e),
            IrAsm::Prt(e) => write!(f, "print v{}", e),
            IrAsm::Inp(e) => write!(f, "v{} = input", e),
            IrAsm::Cst(e, a) => write!(
                f,
                "v{} = {}",
                e,
                a.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            IrAsm::Mov(e, a) => write!(f, "v{} = v{}", e, a),
            IrAsm::Len(e, a) => write!(f, "v{} = len v{}", e, a),
            IrAsm::Read(a, b, c, d) => write!(f, "v{} = v{}[v{} > v{}]", a, b, c, d),
            IrAsm::Nop => write!(f, ""),
            IrAsm::Meta(e) => write!(f, "meta {}", e),
        }
    }
}

#[derive(Default)]
pub struct Counter(usize);

impl Counter {
    fn get(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

impl IrAsm {
    pub fn to_asm(
        self,
        counter: &mut Counter,
        fors: &mut Vec<usize>,
        func_ret: Option<(usize, usize)>, /* Function ret, Function id */
    ) -> Vec<Asm> {
        vec![match self {
            IrAsm::Op(a, b, c, d) => Asm::Op(a, b, c, d),
            IrAsm::End => Asm::End,
            IrAsm::If(a, b, c, d) => {
                let if_id = counter.get();
                let mut out = Vec::new();
                if c == d {
                    return c
                        .into_iter()
                        .map(|x| x.to_asm(counter, fors, func_ret))
                        .flatten()
                        .collect();
                }
                if c.len() == 0 && d.len() == 0 {
                    return vec![];
                }
                if c.len() == 0 {
                    out.push(Asm::If(true, a, b));
                    out.push(Asm::GtLabel(format!("if_end{}", if_id)));
                    out.extend(
                        d.into_iter()
                            .map(|x| x.to_asm(counter, fors, func_ret))
                            .flatten(),
                    );
                    out.push(Asm::Label(format!("if_end{}", if_id)));
                } else if d.len() == 0 {
                    out.push(Asm::If(false, a, b));
                    out.push(Asm::GtLabel(format!("if_end{}", if_id)));
                    out.extend(
                        c.into_iter()
                            .map(|x| x.to_asm(counter, fors, func_ret))
                            .flatten(),
                    );
                    out.push(Asm::Label(format!("if_end{}", if_id)));
                } else {
                    out.push(Asm::If(false, a, b));
                    out.push(Asm::GtLabel(format!("if_else{}", if_id)));
                    out.extend(
                        c.into_iter()
                            .map(|x| x.to_asm(counter, fors, func_ret))
                            .flatten(),
                    );
                    out.push(Asm::GtLabel(format!("if_end{}", if_id)));
                    out.push(Asm::Label(format!("if_else{}", if_id)));
                    out.extend(
                        d.into_iter()
                            .map(|x| x.to_asm(counter, fors, func_ret))
                            .flatten(),
                    );
                    out.push(Asm::Label(format!("if_end{}", if_id)));
                }
                return out;
            }
            IrAsm::Loop(e) => {
                let if_id = counter.get();
                fors.push(if_id);
                let mut out = Vec::new();
                out.push(Asm::Label(format!("loop_start{}", if_id)));
                out.extend(
                    e.into_iter()
                        .map(|x| x.to_asm(counter, fors, func_ret))
                        .flatten(),
                );
                out.push(Asm::GtLabel(format!("loop_start{}", if_id)));
                out.push(Asm::Label(format!("loop_end{}", if_id)));
                fors.pop();
                return out;
            }
            IrAsm::Break() => Asm::GtLabel(format!("loop_end{}", fors.last().unwrap())),
            IrAsm::Continue() => Asm::GtLabel(format!("loop_start{}", fors.last().unwrap())),
            IrAsm::Prt(d) => Asm::Prt(d),
            IrAsm::Inp(d) => Asm::Inp(d),
            IrAsm::Cst(c, d) => Asm::Cst(c, d),
            IrAsm::Mov(c, d) => Asm::Mov(c, d),
            IrAsm::Len(c, d) => Asm::Len(c, d),
            IrAsm::Read(a, b, c, d) => Asm::Read(a, b, c, d),
            IrAsm::Nop => Asm::Nop,
            IrAsm::Return(e) => {
                return vec![
                    Asm::Mov(func_ret.unwrap().0, e),
                    Asm::GtLabel(format!("func_end{}", func_ret.unwrap().1)),
                ]
            }
            IrAsm::FunctionBlock(into, block) => {
                let if_id = counter.get();
                let mut out = Vec::new();
                out.extend(
                    block
                        .into_iter()
                        .map(|x| x.to_asm(counter, fors, Some((into, if_id))))
                        .flatten(),
                );
                out.push(Asm::Label(format!("func_end{}", if_id)));
                return out;
            }
            IrAsm::Meta(_) => return vec![],
        }]
    }

    fn get_write(&self) -> Option<usize> {
        match self {
            IrAsm::Op(_, a, ..)
            | IrAsm::Inp(a)
            | IrAsm::Cst(a, _)
            | IrAsm::Mov(a, _)
            | IrAsm::Len(a, _)
            | IrAsm::Read(a, _, _, _) => Some(*a),
            _ => None,
        }
    }
}

fn get_what_to_elide(i: &Vec<IrAsm>, set: &mut HashMap<usize, usize>) {
    fn add(i: &usize, set: &mut HashMap<usize, usize>) {
        if let Some(e) = set.get_mut(i) {
            *e += 1;
        } else {
            set.insert(*i, 1);
        }
    }
    for i in i {
        match i {
            IrAsm::Op(_, _, a, b) => {
                add(a, set);
                add(b, set);
            }
            IrAsm::End => {}
            IrAsm::If(a, b, c, d) => {
                add(a, set);
                add(b, set);
                get_what_to_elide(c, set);
                get_what_to_elide(d, set);
            }
            IrAsm::Loop(c) => {
                get_what_to_elide(c, set);
            }
            IrAsm::Break() => {}
            IrAsm::Continue() => {}
            IrAsm::FunctionBlock(_, c) => {
                get_what_to_elide(c, set);
            }
            IrAsm::Return(a) => {
                add(a, set);
            }
            IrAsm::Prt(a) => {
                add(a, set);
            }
            IrAsm::Inp(_) => {}
            IrAsm::Cst(_, _) => {}
            IrAsm::Mov(_, a) => {
                add(a, set);
            }
            IrAsm::Len(_, a) => {
                add(a, set);
            }
            IrAsm::Read(_, a, b, c) => {
                add(a, set);
                add(b, set);
                add(c, set);
            }
            IrAsm::Nop => {}
            IrAsm::Meta(_) => {}
        }
    }
}

pub fn elide_unused_writes(i: Vec<IrAsm>) -> Vec<IrAsm> {
    let mut elidable = HashMap::new();
    get_what_to_elide(&i, &mut elidable);
    fn rec(i: Vec<IrAsm>, elidable: &HashMap<usize, usize>) -> Vec<IrAsm> {
        i.into_iter()
            .flat_map(|i| {
                if let Some(e) = i.get_write() {
                    if !elidable.contains_key(&e) {
                        return None;
                    }
                }
                Some(match i {
                    IrAsm::If(a, b, c, d) => IrAsm::If(a, b, rec(c, elidable), rec(d, elidable)),
                    IrAsm::Loop(a) => IrAsm::Loop(rec(a, elidable)),
                    IrAsm::FunctionBlock(a, b) => IrAsm::FunctionBlock(a, rec(b, elidable)),
                    i => i,
                })
            })
            .collect()
    }
    rec(i, &elidable)
}

fn count_refs(i: &IrAsm, map: &mut HashMap<usize, (usize, usize)>) {
    let mut add = |var: &usize, is_write: bool| {
        if is_write {
            if let Some((a, _)) = map.get_mut(var) {
                *a += 1;
            } else {
                map.insert(*var, (1, 0));
            }
        } else {
            if let Some((_, b)) = map.get_mut(var) {
                *b += 1;
            } else {
                map.insert(*var, (0, 1));
            }
        }
    };
    match i {
        IrAsm::Op(_, a, b, c) => {
            add(a, true);
            add(b, false);
            add(c, false);
        }
        IrAsm::End => {}
        IrAsm::If(a, b, c, d) => {
            add(a, false);
            add(b, false);
            c.iter().for_each(|x| count_refs(x, map));
            d.iter().for_each(|x| count_refs(x, map));
        }
        IrAsm::Loop(c) => {
            c.iter().for_each(|x| count_refs(x, map));
        }
        IrAsm::Break() => {}
        IrAsm::Continue() => {}
        IrAsm::FunctionBlock(a, c) => {
            add(a, true);
            c.iter().for_each(|x| count_refs(x, map));
        }
        IrAsm::Return(e) | IrAsm::Prt(e) => {
            add(e, false);
        }
        IrAsm::Inp(a) | IrAsm::Cst(a, _) => {
            add(a, true);
        }
        IrAsm::Mov(a, b) | IrAsm::Len(a, b) => {
            add(a, true);
            add(b, false);
        }
        IrAsm::Read(a, b, c, d) => {
            add(a, true);
            add(b, false);
            add(c, false);
            add(d, false);
        }
        IrAsm::Nop => {}
        IrAsm::Meta(_) => {}
    }
}

pub fn remove_followed_usages(
    i: Vec<IrAsm>,
    set: Option<Rc<HashMap<usize, (usize, usize)>>>,
) -> Vec<IrAsm> {
    let refs = if let Some(e) = set {
        e
    } else {
        let mut map = HashMap::new();
        i.iter().for_each(|x| count_refs(x, &mut map));
        Rc::new(map)
    };
    if i.len() < 2 {
        return i;
    }
    let mut i = i.into_iter();
    let mut metas = Vec::new();
    let mut last = loop {
        match i.next() {
            Some(IrAsm::Meta(a)) => {
                metas.push(IrAsm::Meta(a));
            }
            Some(e) => break e,
            None => return metas,
        }
    };
    let mut out = Vec::with_capacity(i.len());
    while let Some(mut e) = i.next() {
        e = match e {
            IrAsm::If(a, b, c, d) => IrAsm::If(
                a,
                b,
                remove_followed_usages(c, Some(refs.clone())),
                remove_followed_usages(d, Some(refs.clone())),
            ),
            IrAsm::Loop(a) => IrAsm::Loop(remove_followed_usages(a, Some(refs.clone()))),
            IrAsm::FunctionBlock(a, b) => {
                IrAsm::FunctionBlock(a, remove_followed_usages(b, Some(refs.clone())))
            }
            IrAsm::Meta(e) => {
                metas.push(IrAsm::Meta(e));
                continue;
            }
            e => e,
        };
        if let (IrAsm::Mov(a, b), Some(c)) = (&e, last.get_write()) {
            if *b == c {
                if let Some((writes, reads)) = refs.get(&b) {
                    if *writes == 1 && *reads == 1 {
                        let to_use = *a;
                        last = match last {
                            IrAsm::Op(a, _, c, d) => IrAsm::Op(a, to_use, c, d),
                            IrAsm::End => IrAsm::End,
                            IrAsm::If(a, b, c, d) => IrAsm::If(a, b, c, d),
                            IrAsm::Loop(a) => IrAsm::Loop(a),
                            IrAsm::Break() => IrAsm::Break(),
                            IrAsm::Continue() => IrAsm::Continue(),
                            IrAsm::FunctionBlock(_, b) => IrAsm::FunctionBlock(to_use, b),
                            IrAsm::Return(a) => IrAsm::Return(a),
                            IrAsm::Prt(a) => IrAsm::Prt(a),
                            IrAsm::Inp(_) => IrAsm::Inp(to_use),
                            IrAsm::Cst(_, b) => IrAsm::Cst(to_use, b),
                            IrAsm::Mov(_, b) => IrAsm::Mov(to_use, b),
                            IrAsm::Len(_, b) => IrAsm::Len(to_use, b),
                            IrAsm::Read(_, b, c, d) => IrAsm::Read(to_use, b, c, d),
                            IrAsm::Nop => IrAsm::Nop,
                            IrAsm::Meta(_) => unreachable!(),
                        };
                        continue;
                    }
                }
            }
        }
        out.push(last);
        out.append(&mut metas);
        last = e;
    }
    out.push(last);
    out.append(&mut metas);
    out
}
