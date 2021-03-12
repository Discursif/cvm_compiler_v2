use std::collections::HashMap;

type Variable = usize;

#[derive(Debug, Clone)]
pub enum Instruction {
    Label(String),
    Op(OperationType, Variable, Variable, Variable),
    Gt(usize),
    GtLabel(String),
    End,
    If(bool, Variable, Variable), // bool = inverted?
    Prt(Variable),
    Inp(Variable),
    Cst(Variable, Vec<u8>),
    Mov(Variable, Variable),
    Len(Variable, Variable),
    Read(Variable, Variable, Variable, Variable),
    Nop,
}

impl Instruction {

    pub fn clean(mut selfs: Vec<Instruction>) -> Vec<Self> {
        let labels_pos: HashMap<String, usize> = selfs.iter_mut().enumerate().flat_map(|(line,x)| {
            let o = match x {
                Instruction::Label(e) => e.to_owned(),
                _ => return None
            };
            *x = Instruction::Nop;
            Some((o, line))
        }).collect();

        selfs.iter_mut().for_each(|x| {
            let o = if let Instruction::GtLabel(label) = x {
                if let Some(e) = labels_pos.get(label) {
                    *e
                } else {
                    panic!("Invalid label {}",label)
                }
            } else {
                return;
            };
            *x = Instruction::Gt(o);
        });

        selfs
    }

    pub fn to_raw(&self) -> String {
        match &self {
            Instruction::Label(_) => unreachable!(),
            Instruction::Op(a, b, c, d) => format!("{} v{}, v{}, v{}",a.to_raw(),b,c,d),
            Instruction::Gt(e) => format!("JUMP {}",e),
            Instruction::GtLabel(_) => unreachable!(),
            Instruction::End => "END".to_owned(),
            Instruction::If(a, b, c) => format!("IF{} {}, {}",if *a {"N"} else { ""},b,c),
            Instruction::Prt(a) => format!("PRINT v{}",a),
            Instruction::Inp(a) => format!("INPUT v{}",a),
            Instruction::Cst(a, b) => format!("CONST v{}, {}",a,b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),
            Instruction::Mov(a, b) => format!("MOV v{}, v{}",a,b),
            Instruction::Len(a, b) => format!("LEN v{}, v{}",a,b),
            Instruction::Read(a, b, c, d) => format!("READ v{}, v{}, v{}, v{}",a,b,c,d),
            Instruction::Nop => format!("NO_OP")
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Xor,
    Merge,
}

impl OperationType {

    fn to_raw(&self) -> &'static str {
        match self {
            OperationType::Add => "ADD",
            OperationType::Sub => "SUB",
            OperationType::Mul => "MUL",
            OperationType::Div => "DIV",
            OperationType::Mod => "MOD",
            OperationType::Xor => "XOR",
            OperationType::Merge => "MERGE",
        }
    }
}