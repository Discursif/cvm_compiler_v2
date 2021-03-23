use std::{collections::{HashMap, HashSet}, fmt::Display, rc::Rc};

use crate::asm::{Asm, OperationType};

type Variable = usize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrAsm {
    Op(OperationType, Variable, Variable, Variable),
    End,
    If(Variable, Variable, Vec<Self>, Vec<Self>),
    Loop(Vec<Self>),
    Break(),
    Continue(),
    FunctionBlock(Variable,Vec<Self>),
    Return(Variable),
    Prt(Variable),
    Inp(Variable),
    Cst(Variable, Vec<u8>),
    Mov(Variable, Variable),
    Len(Variable, Variable),
    Read(Variable, Variable, Variable, Variable),
    Nop,
}

impl Display for IrAsm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrAsm::Op(a, b, c, d) => write!(f,"v{} = v{} {} v{}",b,c,a.as_operator(),d),
            IrAsm::End => write!(f,"end"),
            IrAsm::If(a, b, c, d) => {
                write!(f,"if v{} == v{} {{\n  {}\n}} else {{\n  {}\n}}",a,b,c.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n").replace("\n","\n  "),d.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n").replace("\n","\n  "))
            }
            IrAsm::Loop(e) => {
                write!(f,"loop {{\n  {}\n}}",e.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n").replace("\n","\n  "))
            }
            IrAsm::Break() => write!(f,"break"),
            IrAsm::Continue() => write!(f,"continue"),
            IrAsm::FunctionBlock(e, a) => {
                write!(f,"v{} = fn {{\n  {}\n}}",e, a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n").replace("\n","\n  "))
            }
            IrAsm::Return(e) => write!(f,"return v{}",e),
            IrAsm::Prt(e) => write!(f,"print v{}",e),
            IrAsm::Inp(e) => write!(f,"v{} = input",e),
            IrAsm::Cst(e, a) => write!(f,"v{} = {}",e,a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),
            IrAsm::Mov(e, a) => write!(f,"v{} = v{}",e,a),
            IrAsm::Len(e, a) => write!(f,"v{} = len v{}",e,a),
            IrAsm::Read(a, b, c, d) => write!(f,"v{} = v{}[v{} > v{}]",a,b,c,d),
            IrAsm::Nop => write!(f,""),
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

    // fn count_reads(&self, var: usize) -> usize {
    //     let ie = |x: &usize| -> usize {
    //         if *x == var {
    //             1
    //         } else {
    //             0
    //         }
    //     };
    //     match self {
    //         IrAsm::Op(_, _, a, b)  => ie(a) + ie(b),
    //         IrAsm::If(a, b, c, d) => {
    //             ie(a) + ie(b) + c.iter().map(|x| x.count_reads(var)).sum::<usize>() + d.iter().map(|x| x.count_reads(var)).sum::<usize>()
    //         },
    //         IrAsm::End | IrAsm::Break() | IrAsm::Continue() | IrAsm::Inp(_) | IrAsm::Cst(_, _) | IrAsm::Nop => 0,
    //         IrAsm::Loop(e) => {
    //             e.iter().map(|x| x.count_reads(var)).sum()
    //         },
    //         IrAsm::FunctionBlock(_, e) => {
    //             e.iter().map(|x| x.count_reads(var)).sum()
    //         }
    //         IrAsm::Return(a) | IrAsm::Prt(a) | IrAsm::Mov(_, a) | IrAsm::Len(_, a) => ie(a),
    //         IrAsm::Read(_, a, b, c) => ie(a) + ie(b) + ie(c),
    //     }
    // }

    fn is_inlinable(&self, is_out: bool) -> bool {
        match self {
            IrAsm::Op(_, _, _, _) => true,
            IrAsm::End => true,
            IrAsm::If(_, _, c, d) => {
                for a in c.iter().chain(d.iter()) {
                    if !a.is_inlinable(false) {
                        return false;
                    }
                }
                true
            }
            IrAsm::Loop(e) => {
                for a in e {
                    if !a.is_inlinable(false) {
                        return false;
                    }
                }
                true
            },
            IrAsm::Break() => true,
            IrAsm::Continue() => true,
            IrAsm::FunctionBlock(_, _) => true,
            IrAsm::Return(_) => is_out,
            IrAsm::Prt(_) => true,
            IrAsm::Inp(_) => true,
            IrAsm::Cst(_, _) => true,
            IrAsm::Mov(_, _) => true,
            IrAsm::Len(_, _) => true,
            IrAsm::Read(_, _, _, _) => true,
            IrAsm::Nop => true,
        }
    }

    pub fn to_asm(self, counter: &mut Counter, fors: &mut Vec<usize>,func_ret: Option<(usize,usize)> /* Function ret, Function id */) -> Vec<Asm> {
        vec![match self {
            IrAsm::Op(a, b, c, d) => Asm::Op(a,b,c,d),
            IrAsm::End => Asm::End,
            IrAsm::If(a, b, c, d) => {
                let if_id = counter.get();
                let mut out = Vec::new();
                if c == d {
                    return c.into_iter().map(|x| x.to_asm(counter,fors,func_ret)).flatten().collect();
                }
                if c.len() == 0 && d.len() == 0 {
                    return vec![];
                }
                if c.len() == 0 {
                    out.push(Asm::If(true, a,b));
                    out.push(Asm::GtLabel(format!("if_end{}",if_id)));
                    out.extend(d.into_iter().map(|x| x.to_asm(counter,fors,func_ret)).flatten());
                    out.push(Asm::Label(format!("if_end{}",if_id)));
                } else if d.len() == 0 {
                    out.push(Asm::If(false, a,b));
                    out.push(Asm::GtLabel(format!("if_end{}",if_id)));
                    out.extend(c.into_iter().map(|x| x.to_asm(counter,fors,func_ret)).flatten());
                    out.push(Asm::Label(format!("if_end{}",if_id)));
                } else {
                    out.push(Asm::If(false, a,b));
                    out.push(Asm::GtLabel(format!("if_else{}",if_id)));
                    out.extend(c.into_iter().map(|x| x.to_asm(counter,fors,func_ret)).flatten());
                    out.push(Asm::GtLabel(format!("if_end{}",if_id)));
                    out.push(Asm::Label(format!("if_else{}",if_id)));
                    out.extend(d.into_iter().map(|x| x.to_asm(counter,fors,func_ret)).flatten());
                    out.push(Asm::Label(format!("if_end{}",if_id)));
                }
                return out
            },
            IrAsm::Loop(e) => {
                let if_id = counter.get();
                fors.push(if_id);
                let mut out = Vec::new();
                out.push(Asm::Label(format!("loop_start{}",if_id)));
                out.extend(e.into_iter().map(|x| x.to_asm(counter,fors,func_ret)).flatten());
                out.push(Asm::GtLabel(format!("loop_start{}",if_id)));
                out.push(Asm::Label(format!("loop_end{}",if_id)));
                fors.pop();
                return out;
            }
            IrAsm::Break() => Asm::GtLabel(format!("loop_end{}",fors.last().unwrap())),
            IrAsm::Continue() => Asm::GtLabel(format!("loop_start{}",fors.last().unwrap())),
            IrAsm::Prt(d) => Asm::Prt(d),
            IrAsm::Inp(d) => Asm::Inp(d),
            IrAsm::Cst(c, d) => Asm::Cst(c,d),
            IrAsm::Mov(c, d) => Asm::Mov(c,d),
            IrAsm::Len(c, d) => Asm::Len(c,d),
            IrAsm::Read(a, b, c, d) => Asm::Read(a,b,c,d),
            IrAsm::Nop => Asm::Nop,
            IrAsm::Return(e) => {
                return vec![Asm::Mov(func_ret.unwrap().0,e),Asm::GtLabel(format!("func_end{}",func_ret.unwrap().1))]
            },
            IrAsm::FunctionBlock(into, block) => {
                let if_id = counter.get();
                let mut out = Vec::new();
                out.extend(block.into_iter().map(|x| x.to_asm(counter,fors,Some((into,if_id)))).flatten());
                out.push(Asm::Label(format!("func_end{}",if_id)));
                return out;
            }
        }]
    }

    fn get_write(&self) -> Option<usize> {
        match self {
            IrAsm::Op(_, a, ..) |IrAsm::Inp(a) | IrAsm::Cst(a, _) | IrAsm::Mov(a, _) | IrAsm::Len(a, _) | IrAsm::Read(a, _, _, _) => Some(*a),
            _ => None
        }
    }
}

#[test]
fn test() {
    let code = vec![
        IrAsm::Cst(1, vec![26]),
        IrAsm::Cst(2, vec![30]),
        IrAsm::Cst(3, vec![1]),
        IrAsm::Loop(vec![
            IrAsm::If(1,2,vec![IrAsm::Break()],vec![]),
            IrAsm::Prt(1),
            IrAsm::Op(OperationType::Add,2,2,3)
        ])
    ];
    let mut counter = Counter::default();
    let mut fors= Vec::new();
    let asm = code.into_iter().map(|x| x.to_asm(&mut counter, &mut fors, None)).flatten().collect::<Vec<_>>();
    let asm = Asm::clean(asm);
    asm.iter().for_each(|x| {
        println!("{}",x.to_raw());
    });
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum KnowledgeState {
    Value(Vec<u8>),
    Length(u8),
}

impl KnowledgeState {
    fn get_length(&self) -> Option<u8> {
        match self {
            KnowledgeState::Value(e) => Some(e.len() as u8),
            KnowledgeState::Length(e) => Some(*e),
        }
    }
    fn get_value(&self) -> Option<Vec<u8>> {
        match self {
            KnowledgeState::Value(e) => Some(e.clone()),
            KnowledgeState::Length(_) => None,
        }
    }
}

pub trait Optimizer: Sized {
    fn optimize(self, current_values: VariableManager) -> (Self,VariableManager);
    fn get_muts(&self, a: &mut HashSet<usize>);
}

fn compute(o: &OperationType, vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    match o {
        OperationType::Add => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x.wrapping_add(y)).collect()
        }
        OperationType::And => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x & y).collect()
        }
        OperationType::Sub => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x.wrapping_sub(y)).collect()
        }
        OperationType::Mul => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x.wrapping_mul(y)).collect()
        }
        OperationType::Div => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x.wrapping_div(y)).collect()
        }
        OperationType::Mod => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x % y).collect()
        }
        OperationType::Xor => {
            vec1.into_iter().zip(vec2.into_iter().cycle()).map(|(x,y)| x ^ y).collect()
        }
        OperationType::Merge => {
            vec1.into_iter().chain(vec2.into_iter()).collect()
        }
    }
}

#[derive(Clone, Default)]
pub struct VariableManager(HashMap<usize, KnowledgeState>);

impl VariableManager {
    fn get_var(&self, u: &usize) -> Option<&KnowledgeState> {
        self.0.get(u)
    }
    fn set_var(&mut self, u: &usize, value: KnowledgeState) {
        self.0.insert(*u, value);
    }
    fn clear_var(&mut self, u: &usize) {
        self.0.remove(u);
    }
}

pub fn get_whats_the_same(manager1: VariableManager, manager2: VariableManager) -> VariableManager {
    VariableManager(manager1.0.into_iter().flat_map(|(index, x)| {
        let other = manager2.get_var(&index)?;
        match (&x,other) {
            (KnowledgeState::Value(a), KnowledgeState::Value(b)) => {
                if a == b {
                    Some((index,KnowledgeState::Value(a.clone())))
                } else if a.len() == b.len() {
                    Some((index,KnowledgeState::Length(a.len() as u8)))
                } else {
                    None
                }
            }
            (KnowledgeState::Value(a), KnowledgeState::Length(b)) | (KnowledgeState::Length(b), KnowledgeState::Value(a)) => {
                if a.len() == *b as usize {
                    Some((index,KnowledgeState::Length(*b)))
                } else {
                    None
                }
            }
            (KnowledgeState::Length(a), KnowledgeState::Length(b)) => {
                if a == b {
                    Some((index,KnowledgeState::Length(*b)))
                } else {
                    None
                }
            }
        }
    }).collect())
}

impl Optimizer for Vec<IrAsm> {

    
    fn get_muts(&self, a: &mut HashSet<usize>) {
        self.into_iter().for_each(|x| {
            match x {
                IrAsm::Op(_, b, _, _) => {
                    a.insert(*b);
                },
                IrAsm::End => (),
                IrAsm::If(_, _, e, f) => {
                    e.get_muts(a);
                    f.get_muts(a);
                },
                IrAsm::Loop(e) => e.get_muts(a),
                IrAsm::Break() => {}
                IrAsm::Continue() => {}
                IrAsm::FunctionBlock(b, f) => {
                    a.insert(*b);
                    f.get_muts(a);
                }
                IrAsm::Return(_) => {}
                IrAsm::Prt(_) => {}
                IrAsm::Inp(b) => {
                    a.insert(*b);
                }
                IrAsm::Cst(b, _) => {
                    a.insert(*b);
                }
                IrAsm::Mov(b, _) => {
                    a.insert(*b);
                }
                IrAsm::Len(b, _) => {
                    a.insert(*b);
                }
                IrAsm::Read(b, _, _, _) => {
                    a.insert(*b);
                }
                IrAsm::Nop => {}
            }
        });
    }

    fn optimize(self, mut current_values: VariableManager) -> (Self,VariableManager) {
        let mut was_breaked = false;
        (self.into_iter().map(|x| {
            if was_breaked {
                return vec![IrAsm::Nop];
            }
            match &x {
                IrAsm::Op(o, a, b, c) => {
                    match (current_values.get_var(b),current_values.get_var(c)) {
                        (None, _) => {}
                        (Some(e), Some(b)) => {
                            if let (Some(e), Some(i)) = (e.get_value(),b.get_value()) {
                                let computed = compute(o, e, i);
                                current_values.set_var(a,KnowledgeState::Value(computed.clone()));
                                return vec![IrAsm::Cst(*a,computed)];
                            }
                            if let Some(e) = e.get_length() {
                                current_values.set_var(a,KnowledgeState::Length(e));
                                return vec![x];
                            }
                        }
                        (Some(e), None) => {
                            if let Some(e) = e.get_length() {
                                current_values.set_var(a,KnowledgeState::Length(e));
                                return vec![x];
                            }
                        }
                    }
                    current_values.clear_var(a);
                }
                IrAsm::End => {}
                IrAsm::If(a, b, c, d) => {
                    match (current_values.get_var(a),current_values.get_var(b)) {
                        (Some(KnowledgeState::Length(a)), Some(KnowledgeState::Length(b))) => {
                            if a != b {
                                let (i,after) = d.clone().optimize(current_values.clone());
                                current_values = after;
                                return i;
                            }
                        },
                        (Some(KnowledgeState::Length(a)), Some(KnowledgeState::Value(b))) | 
                        (Some(KnowledgeState::Value(b)), Some(KnowledgeState::Length(a))) => {
                            if *a as usize != b.len() {
                                let (i,after) = d.clone().optimize(current_values.clone());
                                current_values = after;
                                return i;
                            }
                        },
                        (Some(KnowledgeState::Value(a)), Some(KnowledgeState::Value(b))) => {
                            if a != b {
                                let (i,after) = d.clone().optimize(current_values.clone());
                                current_values = after;
                                return i;
                            } else {
                                let (i,after) = c.clone().optimize(current_values.clone());
                                current_values = after;
                                return i;
                            }
                        },
                        _ => ()
                    }

                    let (block1,var1) = c.clone().optimize(current_values.clone());
                    let (block2,var2) = d.clone().optimize(current_values.clone());
                    
                    current_values = get_whats_the_same(var1, var2);
                    return vec![IrAsm::If(*a,*b,block1,block2)];
                }
                IrAsm::Loop(e) => {
                    let before = current_values.clone();
                    let mut out = HashSet::new();
                    e.get_muts(&mut out);
                    current_values = VariableManager(before.0.into_iter().filter(|(x,_)| !out.contains(x)).collect());
                    return vec![IrAsm::Loop(e.clone().optimize(current_values.clone()).0)];
                }
                IrAsm::Break() => {
                    was_breaked = true;
                }
                IrAsm::Continue() => {
                    was_breaked = true;
                }
                IrAsm::Prt(_) => {}
                IrAsm::Inp(a) => {
                    current_values.clear_var(a);
                }
                IrAsm::Cst(a, b) => {
                    current_values.set_var(a,KnowledgeState::Value(b.clone()));
                }
                IrAsm::Mov(a, b) => {
                    match current_values.get_var(b).map(|x| x.clone()) {
                        None => {
                            current_values.clear_var(a);
                        },
                        Some(KnowledgeState::Length(c)) => {
                            current_values.set_var(a,KnowledgeState::Length(c));
                        },
                        Some(KnowledgeState::Value(c)) => {
                            current_values.set_var(a,KnowledgeState::Value(c.clone()));
                            return vec![IrAsm::Cst(*a,c.clone())];
                        }
                    }
                }
                IrAsm::Len(a, b) => {
                    if let Some(e) = current_values.get_var(b).map(|x| x.get_length()).flatten() {
                        current_values.set_var(a,KnowledgeState::Value(vec![e]));
                        return vec![IrAsm::Cst(*a,vec![e])];
                    } else {
                        current_values.clear_var(a);
                    }
                }
                IrAsm::Read(a, b, c, d) => {
                    if let (Some(b), Some(c), Some(d)) = (current_values.get_var(b).map(|x| x.get_value()).flatten(),
                        current_values.get_var(c).map(|x| x.get_value()).flatten(),
                        current_values.get_var(d).map(|x| x.get_value()).flatten()) {
                            let computed: Vec<u8> = b.into_iter().skip(c[0] as usize).take(d[0] as usize).collect();
                            current_values.set_var(a,KnowledgeState::Value(computed.clone()));
                            return vec![IrAsm::Cst(*a,computed)];
                    } else {
                        current_values.clear_var(a);
                    }
                }
                IrAsm::Nop => return vec![],
                IrAsm::FunctionBlock(e, i) => {
                    if !i.iter().any(|x| !x.is_inlinable(true)) {
                        let mut p = false;
                        let mut i = i.clone();
                        i.iter_mut().for_each(|x| {
                            if p {
                                *x = IrAsm::Nop;
                                return;
                            }
                            match &*x {
                                IrAsm::Return(i) => {
                                    *x = IrAsm::Mov(*e,*i);
                                    p = true;
                                },
                                _ => ()
                            }
                        });
                        let (instr, after) =  i.clone().optimize(current_values.clone());
                        current_values = after;
                        return instr;
                    }
                    let (instr, after) =  i.clone().optimize(current_values.clone());
                    current_values = after;
                    current_values.clear_var(e);
                    return vec![IrAsm::FunctionBlock(*e,instr)];
                }
                IrAsm::Return(_) => {
                    was_breaked = true;
                }
            }
            vec![x]
        }).flatten().filter(|x| !matches!(x, IrAsm::Nop)).collect(),current_values)
    }
}

pub fn get_what_to_elide(i: &Vec<IrAsm>, set: &mut HashMap<usize,usize>) {
    fn add(i: &usize,set: &mut HashMap<usize,usize>) {
        if let Some(e) = set.get_mut(i) {
            *e += 1;
        } else {
            set.insert(*i, 1);
        }
    }
    for i in i {
        match i {
            IrAsm::Op(_, _, a, b) => {
                add(a,set);
                add(b,set);
            }
            IrAsm::End => {
            }
            IrAsm::If(a, b, c, d) => {
                add(a,set);
                add(b,set);
                get_what_to_elide(c,set);
                get_what_to_elide(d,set);
            }
            IrAsm::Loop(c) => {
                get_what_to_elide(c,set);
            }
            IrAsm::Break() => {}
            IrAsm::Continue() => {}
            IrAsm::FunctionBlock(_, c) => {
                get_what_to_elide(c,set);
            }
            IrAsm::Return(a) => {
                add(a,set);
            }
            IrAsm::Prt(a) => {
                add(a,set);
            }
            IrAsm::Inp(_) => {}
            IrAsm::Cst(_, _) => {}
            IrAsm::Mov(_, a) => {
                add(a,set);
            }
            IrAsm::Len(_, a) => {
                add(a,set);
            }
            IrAsm::Read(_, a, b, c) => {
                add(a,set);
                add(b,set);
                add(c,set);
            }
            IrAsm::Nop => {}
        }
    }
}

pub fn elide_unused_writes(i: Vec<IrAsm>) -> Vec<IrAsm> {
    let mut elidable = HashMap::new();
    get_what_to_elide(&i, &mut elidable);
    fn rec(i: Vec<IrAsm>, elidable: &HashMap<usize,usize>) -> Vec<IrAsm> {
        i.into_iter().flat_map(|i| {
            if let Some(e) = i.get_write() {
                if !elidable.contains_key(&e) {
                    return None;
                }
            }
            Some(match i {
                IrAsm::If(a, b, c, d) => IrAsm::If(a, b, rec(c,elidable), rec(d,elidable)),
                IrAsm::Loop(a) => IrAsm::Loop(rec(a,elidable)),
                IrAsm::FunctionBlock(a, b) => IrAsm::FunctionBlock(a, rec(b,elidable)),
                i => i
            })
        }).collect()
    }
    rec(i,&elidable)
}

// pub fn elide_unused_consts(i: Vec<IrAsm>, set: Option<Rc<HashSet<usize>>>) -> Vec<IrAsm> {
//     let refs = if let Some(e) = set {
//         e
//     } else {
//         Rc::new(i.iter().flat_map(|x| {
//             let write= x.get_write()?;
//             if i.iter().any(|x| x.count_reads(write) != 0) {
//                 None
//             } else {
//                 Some(write)
//             }
//         }).collect())
//     };
//     i.into_iter().flat_map(|x| {
//         if let Some(e) = x.get_write() {
//             if refs.contains(&e) {
//                 return None;
//             }
//         }
//         Some(match x {
//             IrAsm::If(a, b, c, d) => {
//                 IrAsm::If(a, b, elide_unused_consts(c, Some(refs.clone())), elide_unused_consts(d, Some(refs.clone())))
//             }
//             IrAsm::Loop(e) => IrAsm::Loop(elide_unused_consts(e, Some(refs.clone()))),
//             IrAsm::FunctionBlock(a, e) => IrAsm::FunctionBlock(a,elide_unused_consts(e, Some(refs.clone()))),
//             e => e
//         })
//     }).collect()
// }

pub fn count_refs(i: &IrAsm, map: &mut HashMap<usize,(usize,usize)>) {
    let mut add = |var: &usize, is_write: bool| {
        if is_write {
            if let Some((a,_)) = map.get_mut(var) {
                *a += 1;
            } else {
                map.insert(*var,(1,0));
            }
        } else {
            if let Some((_,b)) = map.get_mut(var) {
                *b += 1;
            } else {
                map.insert(*var,(0,1));
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
    }
}

pub fn remove_followed_usages(i: Vec<IrAsm>, set: Option<Rc<HashMap<usize,(usize,usize)>>>) -> Vec<IrAsm>{
    let refs = if let Some(e) = set {
        e
    } else {
        let mut map = HashMap::new();
        i.iter().for_each(|x| count_refs(x,&mut map));
        Rc::new(map)
    };
    if i.len() < 2 {
        return i;
    }
    let mut i = i.into_iter();
    let mut last = i.next().unwrap();
    let mut out = Vec::with_capacity(i.len());
    while let Some(mut e) = i.next() {
        e = match e {
            IrAsm::If(a, b, c, d) => {
                IrAsm::If(a, b, remove_followed_usages(c,Some(refs.clone())), remove_followed_usages(d,Some(refs.clone())))
            }
            IrAsm::Loop(a) => {
                IrAsm::Loop(remove_followed_usages(a,Some(refs.clone())))
            }
            IrAsm::FunctionBlock(a, b) => {
                IrAsm::FunctionBlock(a,remove_followed_usages(b,Some(refs.clone())))
            }
            e => e
        };
        if let (IrAsm::Mov(a,b), Some(c)) = (&e,last.get_write()) {
            if *b == c {
                if let Some((writes,reads)) = refs.get(&b) {
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
                            IrAsm::Nop => IrAsm::Nop
                        };
                        continue;
                    }
                }
            }
        }
        out.push(last);
        last = e;
    }
    out.push(last);
    out
}