use std::collections::{HashMap, HashSet};

use crate::asm::OperationType;

use super::{IrAsm, Optimizer};

#[derive(Clone, Debug, PartialEq)]
pub enum KnowledgeState {
    Unknown,
    Equivalent(usize),
    Length(usize),
    Value(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct MemoryManager(pub HashMap<usize, KnowledgeState>);

impl MemoryManager {
    fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_what_is_the_same(&self, manager2: &Self) -> Self {
        let mut newstate: HashMap<usize, KnowledgeState> = HashMap::new();
        for i in self.0.keys() {
            if let (Some(a), Some(b)) = (manager2.get_value(i),self.get_value(i)) {
                if a == b {
                    newstate.insert(*i, KnowledgeState::Value(a));
                } else {
                    newstate.insert(*i, KnowledgeState::Unknown);
                }
            } else if let (Some(a), Some(b)) = (manager2.get_length(i),self.get_length(i)) {
                if a == b {
                    newstate.insert(*i, KnowledgeState::Length(a));
                } else {
                    newstate.insert(*i, KnowledgeState::Unknown);
                }
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
            },
            _ => variable,
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
            Some(KnowledgeState::Equivalent(e)) => {
                self.get_raw(&e)
            }
            e => e
        }.unwrap_or(&KnowledgeState::Unknown)
    }

    pub fn get_value(&self, variable: &usize) -> Option<Vec<u8>> {
        match self.0.get(variable) {
            Some(KnowledgeState::Unknown) | None | Some(KnowledgeState::Length(_)) => None,
            Some(KnowledgeState::Equivalent(a)) => {
                if a == variable {
                    return None;
                }
                self.get_value(a)
            },
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
            },
            Some(KnowledgeState::Length(a)) => Some(*a),
            Some(KnowledgeState::Value(a)) => Some(a.len()),
        }
    }
}

pub fn optimize(data: Vec<IrAsm>) -> Vec<IrAsm> {
    let mut manager = MemoryManager::new();
    data.into_iter().map(|x| update(x, &mut manager)).flatten().collect()
}

fn update(mut operation: IrAsm, state: &mut MemoryManager) -> Vec<IrAsm> {
    let mut no = None;
    match &mut operation {
        IrAsm::Op(operation_type, into, reg1, reg2) => {
            *reg1 = state.get_alias(*reg1);
            *reg2 = state.get_alias(*reg2);
            match (state.get_flattened(&reg1), state.get_flattened(&reg2)) {
                (KnowledgeState::Value(value), KnowledgeState::Unknown) => {
                    if operation_type != &OperationType::Merge {
                        state.set(*into, KnowledgeState::Length(value.len()));
                    } else {
                        state.set(*into, KnowledgeState::Unknown);
                    }
                }
                (KnowledgeState::Length(value), KnowledgeState::Unknown) => {
                    if operation_type != &OperationType::Merge {
                        state.set(*into, KnowledgeState::Length(*value));
                    } else {
                        state.set(*into, KnowledgeState::Unknown);
                    }
                }
                (KnowledgeState::Value(value), KnowledgeState::Value(value1)) => {
                    // TODO: Compute
                    if operation_type == &OperationType::Merge {
                        let mut a = value.clone();
                        a.extend(value1.clone().into_iter());
                        state.set(*into, KnowledgeState::Value(a));
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
                _ => {
                    state.set(*into, KnowledgeState::Unknown);
                }
            }
        }
        IrAsm::End => {}
        IrAsm::If(a, b, c, d) => {
            *a = state.get_alias(*a);
            *b = state.get_alias(*b);
            match (state.get_flattened(&a),state.get_flattened(&b)) {
                (KnowledgeState::Unknown, _) | (_, KnowledgeState::Unknown) |
                (KnowledgeState::Equivalent(_), KnowledgeState::Length(_)) |
                (KnowledgeState::Equivalent(_), KnowledgeState::Value(_)) |
                (KnowledgeState::Length(_), KnowledgeState::Equivalent(_)) |
                (KnowledgeState::Value(_), KnowledgeState::Equivalent(_)) => {}
                (KnowledgeState::Equivalent(m), KnowledgeState::Equivalent(n)) => {
                    if m == n {
                        return c.into_iter().map(|x| update(x.clone(), state)).flatten().collect();
                    }
                }
                (KnowledgeState::Length(a), KnowledgeState::Length(b)) => {
                    if a != b {
                        return d.into_iter().map(|x| update(x.clone(), state)).flatten().collect();
                    }
                }
                (KnowledgeState::Value(b), KnowledgeState::Length(a)) | (KnowledgeState::Length(a), KnowledgeState::Value(b)) => {
                    if *a != b.len() {
                        return d.into_iter().map(|x| update(x.clone(), state)).flatten().collect();
                    }
                }
                (KnowledgeState::Value(a), KnowledgeState::Value(b)) => {
                    if a == b {
                        return c.into_iter().map(|x| update(x.clone(), state)).flatten().collect();
                    } else {
                        return d.into_iter().map(|x| update(x.clone(), state)).flatten().collect();
                    }
                }
            }
            // CHECK IF VALID LATER
            let mut state1 = state.clone();
            if state1.get_flattened(&*a) != &KnowledgeState::Unknown {
                state1.set(*b, KnowledgeState::Equivalent(*a));
            } else {
                state1.set(*a, KnowledgeState::Equivalent(*b));
            }
            let p = c.into_iter().map(|x| update(x.clone(), &mut state1)).flatten().collect();
            let m = d.into_iter().map(|x| update(x.clone(), &mut *state)).flatten().collect();
            *state = state.get_what_is_the_same(&state1);
            no = Some(vec![IrAsm::If(*a,*b,p,m)]);

        }
        IrAsm::Loop(block) => {
            let mut set = HashSet::new();
            block.get_muts(&mut set);
            for s in set {
                state.0.insert(s, KnowledgeState::Unknown);
            }
            let mut state1 = state.clone();
            no = Some(vec![IrAsm::Loop(block.into_iter().map(|x| update(x.clone(), &mut state1)).flatten().collect())]);
        }
        IrAsm::Break() => {}
        IrAsm::Continue() => {}
        IrAsm::FunctionBlock(a, block) => {
            *block = block.into_iter().map(|x| update(x.clone(), state)).flatten().collect();
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
            if let Some(e) = state.get_value(&b) {
                no = Some(vec![IrAsm::Cst(*a,e.clone())]);
                state.set(*a, KnowledgeState::Value(e));
            } else {
                state.set(*a, KnowledgeState::Equivalent(*b));
            }
        }
        IrAsm::Len(a, b) => {
            *b = state.get_alias(*b);
            if let Some(e) = state.get_length(&b) {
                no = Some(vec![IrAsm::Cst(*a,vec![e as u8])]);
                state.set(*a, KnowledgeState::Value(vec![e as u8]));
            } else {
                //state.set(*a, KnowledgeState::Length(*b));
                state.set(*a, KnowledgeState::Unknown);
            }
        }
        IrAsm::Read(a, b, c, d) => {
            *b = state.get_alias(*b);
            *c = state.get_alias(*c);
            *d = state.get_alias(*d);
            if let (Some(m),Some(n),Some(o)) = (state.get_value(&b),state.get_value(&c),state.get_value(&d)) {
                let v: Vec<u8> = m.into_iter().skip(n[0] as usize).take(o[0] as usize).collect();
                no = Some(vec![IrAsm::Cst(*a,v.clone())]);
                state.set(*a, KnowledgeState::Value(v));
            } else {
                state.set(*a, KnowledgeState::Unknown);
            }
        }
        IrAsm::Nop => {}
    }
    if let Some(e) = no {
        e
    } else {
        vec![operation]
    }
}

#[test]
fn optimize_try() {
    let mut code = vec![IrAsm::Cst(102, vec![7]), IrAsm::Cst(369, vec![32]), IrAsm::Cst(147, vec![9]), IrAsm::Cst(139, vec![73, 110, 118, 97, 108, 105, 100, 32, 105, 110, 112, 117, 116, 10]), IrAsm::Cst(114, vec![6]), IrAsm::Cst(284, vec![237]), IrAsm::Cst(359, vec![32, 104, 97, 115, 32, 119, 111, 110, 33, 10]), IrAsm::Cst(123, vec![69, 110, 116, 101, 114, 32, 97, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 116, 111, 32, 112, 108, 97, 121, 32, 105, 110, 32, 58, 32]), IrAsm::Cst(300, vec![2]), IrAsm::Cst(247, vec![1]), IrAsm::Cst(410, vec![65, 108, 100, 114, 101, 97, 100, 121, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 112, 108, 97, 99, 101, 100, 32, 104, 101, 114, 101, 10]), IrAsm::Cst(224, vec![3]), IrAsm::Cst(98, vec![32, 124, 32]), IrAsm::Cst(281, vec![88]), IrAsm::Cst(394, vec![0]), IrAsm::Cst(180, vec![49]), IrAsm::Cst(89, vec![10]), IrAsm::Cst(273, vec![8]), IrAsm::Cst(61, vec![4]), IrAsm::Cst(146, vec![49, 50, 51, 52, 53, 54, 55, 56, 57]), IrAsm::Cst(49, vec![5]), IrAsm::Cst(292, vec![79]), IrAsm::Cst(84, vec![45, 45, 45, 45, 45, 45, 45, 45, 45, 10]), IrAsm::Cst(400, vec![78, 111, 98, 111, 100, 121, 32, 119, 111, 110, 33, 10]), IrAsm::Cst(2, vec![32, 32, 32, 32, 32, 32, 32, 32, 32]), IrAsm::Cst(4, vec![88]), IrAsm::Loop(vec![IrAsm::Read(9, 2, 300, 247), IrAsm::Op(OperationType::Merge, 13, 9, 89), IrAsm::Op(OperationType::Merge, 17, 98, 13), IrAsm::Read(21, 2, 247, 247), IrAsm::Op(OperationType::Merge, 25, 21, 17), IrAsm::Op(OperationType::Merge, 29, 98, 25), IrAsm::Read(33, 2, 394, 247), IrAsm::Op(OperationType::Merge, 37, 33, 29), IrAsm::Prt(37), IrAsm::Prt(84), IrAsm::Read(50, 2, 49, 247), IrAsm::Op(OperationType::Merge, 54, 50, 89), IrAsm::Op(OperationType::Merge, 58, 98, 54), IrAsm::Read(62, 2, 61, 247), IrAsm::Op(OperationType::Merge, 66, 62, 58), IrAsm::Op(OperationType::Merge, 70, 98, 66), IrAsm::Read(74, 2, 224, 247), IrAsm::Op(OperationType::Merge, 78, 74, 70), IrAsm::Prt(78), IrAsm::Prt(84), IrAsm::Read(91, 2, 273, 247), IrAsm::Op(OperationType::Merge, 95, 91, 89), IrAsm::Op(OperationType::Merge, 99, 98, 95), IrAsm::Read(103, 2, 102, 247), IrAsm::Op(OperationType::Merge, 107, 103, 99), IrAsm::Op(OperationType::Merge, 111, 98, 107), IrAsm::Read(115, 2, 114, 247), IrAsm::Op(OperationType::Merge, 119, 115, 111), IrAsm::Prt(119), IrAsm::Prt(123), IrAsm::Inp(125), IrAsm::Prt(89), IrAsm::Len(131, 125), IrAsm::If(131, 247, vec![IrAsm::Cst(135, vec![0])], vec![IrAsm::Cst(135, vec![1])]), IrAsm::Mov(134, 135), IrAsm::If(134, 247, vec![IrAsm::Prt(139), IrAsm::Continue()], vec![]), IrAsm::Mov(125, 125), IrAsm::FunctionBlock(145, vec![IrAsm::Cst(151, vec![0]), IrAsm::Loop(vec![IrAsm::If(147, 151, vec![IrAsm::Break()], vec![]), IrAsm::Read(153, 146, 151, 247), IrAsm::Op(OperationType::Add, 151, 151, 247), IrAsm::If(153, 125, vec![IrAsm::Cst(164, vec![1])], vec![IrAsm::Cst(164, vec![0])]), IrAsm::Mov(163, 164), IrAsm::If(163, 247, vec![IrAsm::Return(247)], vec![])]), IrAsm::Return(394)]), IrAsm::If(145, 394, vec![IrAsm::Cst(171, vec![1])], vec![IrAsm::Cst(171, vec![0])]), IrAsm::Mov(168, 171), IrAsm::If(168, 247, vec![IrAsm::Prt(139), IrAsm::Continue()], vec![]), IrAsm::Op(OperationType::Sub, 181, 125, 180), IrAsm::FunctionBlock(185, vec![IrAsm::Read(187, 2, 181, 247), IrAsm::If(187, 369, vec![IrAsm::Cst(192, vec![0])], vec![IrAsm::Cst(192, vec![1])]), IrAsm::Mov(191, 192), IrAsm::If(191, 247, vec![IrAsm::Return(247)], vec![]), IrAsm::Len(196, 4), IrAsm::Op(OperationType::Add, 198, 181, 196), IrAsm::Len(202, 2), IrAsm::Read(204, 2, 198, 202), IrAsm::Op(OperationType::Merge, 206, 4, 204), IrAsm::Op(OperationType::Sub, 211, 181, 394), IrAsm::Read(210, 2, 394, 211), IrAsm::Op(OperationType::Merge, 2, 210, 206), IrAsm::FunctionBlock(221, vec![IrAsm::Cst(222, vec![0]), IrAsm::Loop(vec![IrAsm::If(222, 224, vec![IrAsm::Break()], vec![]), IrAsm::Mov(226, 222), IrAsm::Op(OperationType::Add, 222, 222, 247), IrAsm::Read(233, 2, 226, 247), IrAsm::If(233, 369, vec![IrAsm::Cst(238, vec![1])], vec![IrAsm::Cst(238, vec![0])]), IrAsm::Mov(237, 238), IrAsm::If(237, 247, vec![IrAsm::Continue()], vec![]), IrAsm::Op(OperationType::Add, 242, 226, 114), IrAsm::Read(246, 2, 242, 247), IrAsm::Op(OperationType::Add, 251, 226, 224), IrAsm::Read(255, 2, 251, 247), IrAsm::Op(OperationType::Add, 259, 255, 246), IrAsm::Read(263, 2, 226, 247), IrAsm::Op(OperationType::Add, 240, 263, 259), IrAsm::If(240, 273, vec![IrAsm::Cst(279, vec![1])], vec![IrAsm::Cst(279, vec![0])]), IrAsm::Mov(278, 279), IrAsm::If(278, 247, vec![IrAsm::Return(281)], vec![]), IrAsm::If(240, 284, vec![IrAsm::Cst(290, vec![1])], vec![IrAsm::Cst(290, vec![0])]), IrAsm::Mov(289, 290), IrAsm::If(289, 247, vec![IrAsm::Return(292)], vec![]), IrAsm::Op(OperationType::Mul, 293, 226, 224), IrAsm::Op(OperationType::Add, 301, 293, 300), IrAsm::Read(305, 2, 301, 247), IrAsm::Op(OperationType::Add, 310, 293, 247), IrAsm::Read(314, 2, 310, 247), IrAsm::Op(OperationType::Add, 318, 314, 305), IrAsm::Read(322, 2, 293, 247), IrAsm::Op(OperationType::Add, 299, 322, 318), IrAsm::If(299, 273, vec![IrAsm::Cst(338, vec![1])], vec![IrAsm::Cst(338, vec![0])]), IrAsm::Mov(337, 338), IrAsm::If(337, 247, vec![IrAsm::Return(281)], vec![]), IrAsm::If(299, 284, vec![IrAsm::Cst(349, vec![1])], vec![IrAsm::Cst(349, vec![0])]), IrAsm::Mov(348, 349), IrAsm::If(348, 247, vec![IrAsm::Return(292)], vec![])]), IrAsm::Return(369)]), IrAsm::Mov(220, 221), IrAsm::If(220, 369, vec![IrAsm::Cst(355, vec![0])], vec![IrAsm::Cst(355, vec![1])]), IrAsm::Mov(354, 355), IrAsm::If(354, 247, vec![IrAsm::Op(OperationType::Merge, 363, 220, 359), IrAsm::Prt(363), IrAsm::End], vec![]), IrAsm::FunctionBlock(370, vec![IrAsm::Mov(371, 2), IrAsm::Len(372, 371), IrAsm::Cst(376, vec![0]), IrAsm::Loop(vec![IrAsm::If(372, 376, vec![IrAsm::Break()], vec![]), IrAsm::Read(378, 371, 376, 247), IrAsm::Op(OperationType::Add, 376, 376, 247), IrAsm::If(378, 369, vec![IrAsm::Cst(389, vec![1])], vec![IrAsm::Cst(389, vec![0])]), IrAsm::Mov(388, 389), IrAsm::If(388, 247, vec![IrAsm::Return(247)], vec![])]), IrAsm::Return(394)]), IrAsm::If(370, 394, vec![IrAsm::Cst(396, vec![1])], vec![IrAsm::Cst(396, vec![0])]), IrAsm::Mov(393, 396), IrAsm::If(393, 247, vec![IrAsm::Prt(400), IrAsm::End], vec![]), IrAsm::Return(394)]), IrAsm::If(185, 247, vec![IrAsm::Prt(410), IrAsm::Continue()], vec![]), IrAsm::If(4, 292, vec![IrAsm::Cst(417, vec![1])], vec![IrAsm::Cst(417, vec![0])]), IrAsm::Mov(416, 417), IrAsm::If(416, 247, vec![IrAsm::Cst(4, vec![88])], vec![IrAsm::Cst(4, vec![79])])])];
    //println!("{}",code.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
    println!("------------------------------------------------");
    for _ in 0..100 {
        code = optimize(code);
    }
    println!("{}",optimize(code).into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
}