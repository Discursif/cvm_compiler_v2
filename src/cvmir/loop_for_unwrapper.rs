use super::{IrAsm, computer::MemoryManager, fn_inliner};

pub fn optimize_loop(ir: Vec<IrAsm>, state: &MemoryManager) -> Vec<IrAsm>{
    // TODO check h
    //println!("AL -1 {}", ir.iter().take(3).map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
    if let [IrAsm::If(a,b,c,d), e, IrAsm::Op(crate::asm::OperationType::Add,f,g,h),..] = ir.as_slice() {
        //println!("AL 0");
        if state.get_value(h) != Some(vec![1]) {
            return vec![IrAsm::Loop(ir)];
        }
        if !fn_inliner::does_end_in_any_case(c) {
            //println!("AL 1");
            return vec![IrAsm::Loop(ir)];
        }
        if !d.is_empty() {
            //println!("AL 2");
            return vec![IrAsm::Loop(ir)];
        }
        if f != g {
            //println!("AL 3");
            return vec![IrAsm::Loop(ir)];
        }
        
        let constant = if a == f {
            //println!("AL 4");
            b
        } else if b == f {
            //println!("AL 5");
            a
        } else {
            //println!("AL 6");
            return vec![IrAsm::Loop(ir)];
        };
        let default = if let Some(e) = state.get_value(f) {
            //println!("AL 7");
            e
        } else {
            //println!("AL 8");
            return vec![IrAsm::Loop(ir)];
        };
        let constant_value = if let Some(e) = state.get_value(constant) {
            //println!("AL 9");
            e
        } else {
            //println!("AL 10");
            return vec![IrAsm::Loop(ir)];
        };

        if has_continue(c) {
            //println!("AL 11");
            return vec![IrAsm::Loop(ir)];
        }

        if has_continues_or_breaks(&ir.iter().skip(1).cloned().collect()) {
            //println!("AL 12");
            return vec![IrAsm::Loop(ir)];
        }

        // CHECK IF THE LAST INSTR OF IF IS BREAK OR RETURN

        let mut out_instr = Vec::new();
        for i in default[0]..constant_value[0] {
            out_instr.push(IrAsm::Cst(*f,vec![i]));
            out_instr.push(e.clone());
            out_instr.extend(ir.iter().skip(3).cloned());
        }
        // Should check for continues or breaks that are no at the end in the if
        // Should check for continues or breaks out the if
        out_instr.append(&mut remove_breaks(c.clone()));
        return out_instr
    }
    return vec![IrAsm::Loop(ir)];
}

fn remove_breaks(vec: Vec<IrAsm>) -> Vec<IrAsm> {
    vec.into_iter().flat_map(|x| {
        match x {
            IrAsm::If(a,b,c,d) => {
                Some(IrAsm::If(a,b,remove_breaks(c),remove_breaks(d)))
            }
            IrAsm::Break() => None,
            e => Some(e)
        }
    }).collect()
}

fn has_continue(vec: &Vec<IrAsm>) -> bool {
    vec.iter().rev().skip(1).any(|x| has_continues_or_breaks(&vec![x.clone()]))
}

fn has_continues_or_breaks(data: &Vec<IrAsm>) -> bool{
    for i in data {
        match i {
            IrAsm::If(a, b, c, d) => {
                if has_continues_or_breaks(c) || has_continues_or_breaks(d) {
                    return true;
                }
            },
            IrAsm::Break() => {
                return true;
            },
            IrAsm::Continue() => {
                return true;
            },
            _ => {}
        }
    }
    false
}