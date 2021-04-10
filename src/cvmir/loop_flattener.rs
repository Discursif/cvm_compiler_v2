use std::collections::HashSet;

use crate::cvmir::{
    computer::{KnowledgeState, MemoryManager},
    IrAsm,
};

pub fn was_able_to_compute(irasm: &[IrAsm], manager: &mut MemoryManager) -> (bool, bool) /* can't compute, ended */
{
    for i in irasm {
        match i {
            IrAsm::End => return (true, true),
            IrAsm::If(a, b, c, d) => {
                if let (Some(e), Some(f)) = (manager.get_value(a), manager.get_value(b)) {
                    if e == f {
                        let (a, b) = was_able_to_compute(c, manager);
                        if !a {
                            return (false, true);
                        }
                        if b {
                            return (true, true);
                        }
                    } else {
                        let (a, b) = was_able_to_compute(d, manager);
                        if !a {
                            return (false, true);
                        }
                        if b {
                            return (true, true);
                        }
                    }
                } else {
                    return (false, true);
                }
            }
            IrAsm::Loop(e) => {
                return (false, true);
            }
            IrAsm::Break() => {
                return (true, true);
            }
            IrAsm::Continue() => {
                return (true, true);
            }
            IrAsm::FunctionBlock(a, b) => {
                return (false, true);
            }
            IrAsm::Return(a) => {
                return (manager.get_value(a).is_some(), true);
            }
            IrAsm::Prt(a) => {
                if manager.get_value(a).is_none() {
                    return (false, true);
                }
            }
            IrAsm::Inp(a) => {
                return (false, true);
            }
            IrAsm::Cst(a, b) => {
                manager.set(*a, KnowledgeState::Value(b.clone()));
            }
            IrAsm::Mov(a, b) => {
                if let Some(e) = manager.get_value(b) {
                    manager.set(*a, KnowledgeState::Value(e));
                } else {
                    return (false, true);
                }
            }
            IrAsm::Len(a, b) => {
                if let Some(e) = manager.get_length(b) {
                    manager.set(*a, KnowledgeState::Value(vec![e as u8]));
                } else {
                    return (false, true);
                }
            }
            IrAsm::Read(a, b, c, d) => {
                if let (Some(e0), Some(e1), Some(e2)) = (
                    manager.get_value(b),
                    manager.get_value(c),
                    manager.get_value(d),
                ) {
                    manager.set(*a, KnowledgeState::Value(e0.into_iter()));
                } else {
                    return (false, true);
                }
            }
            IrAsm::Nop => {}
            IrAsm::Op(a, b, c, d) => {}
        }
    }
    true
}
