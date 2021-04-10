use std::collections::{HashMap, HashSet};

use crate::{asm::OperationType, cli::OptimizerConfig};

use super::{loop_for_unwrapper, IrAsm};

#[derive(Clone, Debug, PartialEq)]
pub enum KnowledgeState {
    Unknown,
    Equivalent(usize),
    Length(usize),
    Value(Vec<u8>),
}

const IF_ELISION: bool = true;
const ELIDE_OPS: bool = true;
const ELIDE_MOV: bool = true;
const ELIDE_LEN: bool = true;

#[derive(Clone, Debug)]
pub struct MemoryManager(pub HashMap<usize, KnowledgeState>);

impl MemoryManager {
    fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_what_is_the_same(&self, manager2: &Self) -> Self {
        let mut newstate: HashMap<usize, KnowledgeState> = HashMap::new();
        for i in self.0.keys() {
            if let (Some(a), Some(b)) = (manager2.get_value(i), self.get_value(i)) {
                if a == b {
                    newstate.insert(*i, KnowledgeState::Value(a));
                } else {
                    newstate.insert(*i, KnowledgeState::Unknown);
                }
            } else if let (Some(a), Some(b)) = (manager2.get_length(i), self.get_length(i)) {
                if a == b {
                    newstate.insert(*i, KnowledgeState::Length(a));
                } else {
                    newstate.insert(*i, KnowledgeState::Unknown);
                }
            } else if manager2.get_alias(*i) == self.get_alias(*i) {
                newstate.insert(*i, KnowledgeState::Equivalent(manager2.get_alias(*i)));
            } else {
                newstate.insert(*i, KnowledgeState::Unknown);
            }
        }
        Self(newstate)
    }

    pub fn get_alias(&self, variable: usize) -> usize {
        match self.0.get(&variable) {
            Some(KnowledgeState::Equivalent(a)) => {
                if *a == variable {
                    return variable;
                }
                self.get_alias(*a)
            }
            _ => variable,
        }
    }

    pub fn try_set_length(&mut self, variable: usize, length: usize) {
        match self.0.get(&variable) {
            None => {
                self.0.insert(variable, KnowledgeState::Length(length));
            }
            Some(&KnowledgeState::Equivalent(e)) => {
                //println!("Going in {}", e);
                if e == variable {
                    self.0.insert(variable, KnowledgeState::Length(length));
                } else {
                    self.try_set_length(e, length);
                }
            }
            Some(KnowledgeState::Unknown) => {
                self.0.insert(variable, KnowledgeState::Length(length));
            }
            _ => (),
        }
    }

    pub fn set(&mut self, variable: usize, value: KnowledgeState) {
        self.0.values_mut().for_each(|x| {
            // TODO: We can maybe copy the equivalent to state instead of setting Unknown
            if let KnowledgeState::Equivalent(e) = x {
                if *e == variable {
                    *x = KnowledgeState::Unknown;
                }
            }
        });
        self.0.insert(variable, value);
    }

    pub fn get_raw(&self, variable: &usize) -> Option<&KnowledgeState> {
        self.0.get(variable)
    }

    pub fn get_flattened(&self, variable: &usize) -> &KnowledgeState {
        match self.get_raw(variable) {
            Some(KnowledgeState::Equivalent(e)) => self.get_raw(&e),
            e => e,
        }
        .unwrap_or(&KnowledgeState::Unknown)
    }

    pub fn get_value(&self, variable: &usize) -> Option<Vec<u8>> {
        match self.0.get(variable) {
            Some(KnowledgeState::Unknown) | None | Some(KnowledgeState::Length(_)) => None,
            Some(KnowledgeState::Equivalent(a)) => {
                if a == variable {
                    return None;
                }
                self.get_value(a)
            }
            Some(KnowledgeState::Value(a)) => Some(a.clone()),
        }
    }

    pub fn get_length(&self, variable: &usize) -> Option<usize> {
        match self.0.get(variable) {
            Some(KnowledgeState::Unknown) | None => None,
            Some(KnowledgeState::Equivalent(a)) => {
                if a == variable {
                    return None;
                }
                self.get_length(a)
            }
            Some(KnowledgeState::Length(a)) => Some(*a),
            Some(KnowledgeState::Value(a)) => Some(a.len()),
        }
    }
}

pub fn optimize(data: Vec<IrAsm>, config: &OptimizerConfig) -> Vec<IrAsm> {
    let mut manager = MemoryManager::new();
    data.into_iter()
        .map(|x| update(x, &mut manager, config))
        .flatten()
        .collect()
}

#[allow(mutable_borrow_reservation_conflict)]
fn update(
    mut operation: IrAsm,
    state: &mut MemoryManager,
    optimizer_config: &OptimizerConfig,
) -> Vec<IrAsm> {
    let mut no = None;
    match &mut operation {
        IrAsm::Op(operation_type, into, reg1, reg2) => {
            *reg1 = state.get_alias(*reg1);
            *reg2 = state.get_alias(*reg2);
            if ELIDE_OPS {
                match (state.get_flattened(&reg1), state.get_flattened(&reg2)) {
                    (KnowledgeState::Value(value), KnowledgeState::Value(value1)) => {
                        // TODO: Compute
                        if operation_type == &OperationType::Merge {
                            let mut a = value.clone();
                            a.extend(value1.clone().into_iter());
                            state.set(*into, KnowledgeState::Value(a));
                            no = Some(vec![IrAsm::Cst(*into, state.get_value(&into).unwrap())]);
                        } else {
                            state.set(
                                *into,
                                KnowledgeState::Value(
                                    value
                                        .iter()
                                        .zip(value1.iter().cycle())
                                        .map(match operation_type {
                                            OperationType::Sub => {
                                                |(x, y): (&u8, &u8)| x.wrapping_sub(*y)
                                            }
                                            OperationType::Add => {
                                                |(x, y): (&u8, &u8)| x.wrapping_add(*y)
                                            }
                                            OperationType::Mul => {
                                                |(x, y): (&u8, &u8)| x.wrapping_mul(*y)
                                            }
                                            OperationType::Div => {
                                                |(x, y): (&u8, &u8)| x.wrapping_div(*y)
                                            }
                                            OperationType::Mod => |(x, y): (&u8, &u8)| x % y,
                                            OperationType::Xor => |(x, y): (&u8, &u8)| x ^ y,
                                            OperationType::And => |(x, y): (&u8, &u8)| x & y,
                                            //OperationType::Or => |(x, y): (&u8, &u8)| x | y,
                                            _ => unreachable!(),
                                        })
                                        .collect(),
                                ),
                            );
                            no = Some(vec![IrAsm::Cst(*into, state.get_value(&into).unwrap())]);
                        }
                    }
                    (KnowledgeState::Value(value), KnowledgeState::Length(value1)) => {
                        if operation_type != &OperationType::Merge {
                            state.set(*into, KnowledgeState::Length(value.len()));
                        } else {
                            state.set(*into, KnowledgeState::Length(value.len() + *value1));
                        }
                    }
                    (KnowledgeState::Length(value1), KnowledgeState::Value(value)) => {
                        if operation_type != &OperationType::Merge {
                            state.set(*into, KnowledgeState::Length(*value1));
                        } else {
                            state.set(*into, KnowledgeState::Length(value.len() + *value1));
                        }
                    }
                    (KnowledgeState::Length(value), KnowledgeState::Length(value1)) => {
                        if operation_type != &OperationType::Merge {
                            state.set(*into, KnowledgeState::Length(*value));
                        } else {
                            state.set(*into, KnowledgeState::Length(*value + *value1));
                        }
                    }
                    (KnowledgeState::Value(value), _) => {
                        if operation_type != &OperationType::Merge {
                            state.set(*into, KnowledgeState::Length(value.len()));
                        } else {
                            state.set(*into, KnowledgeState::Unknown);
                        }
                    }
                    (KnowledgeState::Length(value), _) => {
                        if operation_type != &OperationType::Merge {
                            state.set(*into, KnowledgeState::Length(*value));
                        } else {
                            state.set(*into, KnowledgeState::Unknown);
                        }
                    }
                    _ => {
                        state.set(*into, KnowledgeState::Unknown);
                    }
                }
            } else {
                state.set(*into, KnowledgeState::Unknown);
            }
        }
        IrAsm::End => {}
        IrAsm::If(a, b, c, d) => {
            if c.len() == 0 && d.len() == 0 {
                return vec![];
            }
            *a = state.get_alias(*a);
            *b = state.get_alias(*b);
            if IF_ELISION {
                match (state.get_flattened(&a), state.get_flattened(&b)) {
                    (KnowledgeState::Unknown, _)
                    | (_, KnowledgeState::Unknown)
                    | (KnowledgeState::Equivalent(_), KnowledgeState::Length(_))
                    | (KnowledgeState::Equivalent(_), KnowledgeState::Value(_))
                    | (KnowledgeState::Length(_), KnowledgeState::Equivalent(_))
                    | (KnowledgeState::Value(_), KnowledgeState::Equivalent(_)) => {}
                    (KnowledgeState::Equivalent(m), KnowledgeState::Equivalent(n)) => {
                        if m == n {
                            return c
                                .into_iter()
                                .map(|x| update(x.clone(), state, optimizer_config))
                                .flatten()
                                .collect();
                        }
                    }
                    (KnowledgeState::Length(a), KnowledgeState::Length(b)) => {
                        if a != b {
                            return d
                                .into_iter()
                                .map(|x| update(x.clone(), state, optimizer_config))
                                .flatten()
                                .collect();
                        }
                    }
                    (KnowledgeState::Value(b), KnowledgeState::Length(a))
                    | (KnowledgeState::Length(a), KnowledgeState::Value(b)) => {
                        if *a != b.len() {
                            return d
                                .into_iter()
                                .map(|x| update(x.clone(), state, optimizer_config))
                                .flatten()
                                .collect();
                        }
                    }
                    (KnowledgeState::Value(a), KnowledgeState::Value(b)) => {
                        if a == b {
                            return c
                                .into_iter()
                                .map(|x| update(x.clone(), state, optimizer_config))
                                .flatten()
                                .collect();
                        } else {
                            return d
                                .into_iter()
                                .map(|x| update(x.clone(), state, optimizer_config))
                                .flatten()
                                .collect();
                        }
                    }
                }
            }
            // CHECK IF VALID LATER
            let mut state1 = state.clone();

            let knowledge = if IF_ELISION {
                if state1.get_flattened(&*a) != &KnowledgeState::Unknown {
                    state1.set(*b, KnowledgeState::Equivalent(*a));
                } else {
                    state1.set(*a, KnowledgeState::Equivalent(*b));
                }
                if c.is_empty() && super::fn_inliner::does_end_in_any_case(&d) {
                    Some(match (state1.get_flattened(a), state1.get_flattened(b)) {
                        (KnowledgeState::Value(a), _) | (_, KnowledgeState::Value(a)) => {
                            KnowledgeState::Value(a.clone())
                        }
                        (KnowledgeState::Length(a), _) | (_, KnowledgeState::Length(a)) => {
                            KnowledgeState::Length(*a)
                        }
                        _ => KnowledgeState::Equivalent(state1.get_alias(*b)),
                    })
                } else {
                    None
                }
            } else {
                None
            };
            let p: Vec<IrAsm> = c
                .into_iter()
                .map(|x| update(x.clone(), &mut state1, optimizer_config))
                .flatten()
                .collect();
            let m: Vec<IrAsm> = d
                .into_iter()
                .map(|x| update(x.clone(), &mut *state, optimizer_config))
                .flatten()
                .collect();
            if super::fn_inliner::does_end_in_any_case(&m) {
                *state = state1;
            } else if !super::fn_inliner::does_end_in_any_case(&p) {
                *state = state.get_what_is_the_same(&state1);
            }
            match knowledge {
                Some(KnowledgeState::Equivalent(e)) => {
                    state.set(*a, KnowledgeState::Equivalent(e));
                }
                Some(e) => {
                    state.set(*a, e.clone());
                    state.set(*b, e);
                }
                _ => (),
            }
            no = Some(vec![IrAsm::If(*a, *b, p, m)]);
        }
        IrAsm::Loop(block) => {
            // TODO IMPLEMENT THE CODEX
            let p = loop_for_unwrapper::optimize_loop(block.clone(), state);
            if p.len() == 1 {
                if let IrAsm::Loop(block) = &p[0] {
                    let mut set = HashSet::new();
                    get_muts(block, &mut set);
                    for s in set {
                        state.0.insert(s, KnowledgeState::Unknown);
                    }
                    let mut state1 = state.clone();
                    return vec![IrAsm::Loop(
                        block
                            .into_iter()
                            .map(|x| update(x.clone(), &mut state1, optimizer_config))
                            .flatten()
                            .collect(),
                    )];
                }
            }
            println!("Inlining for");
            return p
                .into_iter()
                .map(|x| update(x, state, optimizer_config))
                .flatten()
                .collect();
        }
        IrAsm::Break() => {}
        IrAsm::Continue() => {}
        IrAsm::FunctionBlock(a, block) => {
            *block = block
                .into_iter()
                .map(|x| update(x.clone(), state, optimizer_config))
                .flatten()
                .collect();
            state.set(*a, KnowledgeState::Unknown);
        }
        IrAsm::Return(a) => {
            *a = state.get_alias(*a);
        }
        IrAsm::Prt(a) => {
            *a = state.get_alias(*a);
        }
        IrAsm::Inp(a) => {
            state.set(*a, KnowledgeState::Unknown);
        }
        IrAsm::Cst(a, b) => {
            state.set(*a, KnowledgeState::Value(b.clone()));
        }
        IrAsm::Mov(a, b) => {
            *b = state.get_alias(*b);
            if ELIDE_MOV {
                if a == b {
                    return vec![];
                }
                if let Some(e) = state.get_value(&b) {
                    no = Some(vec![IrAsm::Cst(*a, e.clone())]);
                    state.set(*a, KnowledgeState::Value(e));
                } else {
                    state.set(*a, KnowledgeState::Equivalent(*b));
                }
            } else {
                state.set(*a, KnowledgeState::Unknown);
            }
        }
        IrAsm::Len(a, b) => {
            *b = state.get_alias(*b);
            if ELIDE_LEN {
                if let Some(e) = state.get_length(&b) {
                    no = Some(vec![IrAsm::Cst(*a, vec![e as u8])]);
                    state.set(*a, KnowledgeState::Value(vec![e as u8]));
                } else {
                    //state.set(*a, KnowledgeState::Length(*b));
                    state.set(*a, KnowledgeState::Unknown);
                }
            } else {
                state.set(*a, KnowledgeState::Unknown);
            }
        }
        IrAsm::Read(a, b, c, d) => {
            *b = state.get_alias(*b);
            *c = state.get_alias(*c);
            *d = state.get_alias(*d);
            if let (Some(m), Some(n), Some(o)) = (
                state.get_value(&b),
                state.get_value(&c),
                state.get_value(&d),
            ) {
                let v: Vec<u8> = m
                    .into_iter()
                    .skip(n[0] as usize)
                    .take(o[0] as usize)
                    .collect();
                no = Some(vec![IrAsm::Cst(*a, v.clone())]);
                state.set(*a, KnowledgeState::Value(v));
            } else {
                state.set(*a, KnowledgeState::Unknown);
            }
        }
        IrAsm::Nop => {}
        IrAsm::Meta(e) => {
            if optimizer_config.infer_sizes_from_meta {
                match e {
                    super::IrAsmMeta::SetLength(a, b) => {
                        state.try_set_length(*a, *b);
                    }
                }
            }
        }
    }
    if let Some(e) = no {
        e
    } else {
        vec![operation]
    }
}

fn get_muts(ir: &Vec<IrAsm>, a: &mut HashSet<usize>) {
    ir.iter().for_each(|x| match x {
        IrAsm::If(_, _, e, f) => {
            get_muts(e, a);
            get_muts(f, a);
        }
        IrAsm::Loop(e) => get_muts(e, a),
        IrAsm::Break()
        | IrAsm::End
        | IrAsm::Continue()
        | IrAsm::Return(_)
        | IrAsm::Prt(_)
        | IrAsm::Nop => {}
        IrAsm::FunctionBlock(b, f) => {
            a.insert(*b);
            get_muts(f, a);
        }
        IrAsm::Op(_, b, _, _)
        | IrAsm::Inp(b)
        | IrAsm::Cst(b, _)
        | IrAsm::Mov(b, _)
        | IrAsm::Len(b, _)
        | IrAsm::Read(b, _, _, _) => {
            a.insert(*b);
        }
        IrAsm::Meta(_) => {}
    });
}
