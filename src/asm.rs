use std::collections::HashMap;

type Variable = usize;

#[derive(Debug, Clone)]
pub enum Asm {
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

impl Asm {
    pub fn clean(mut selfs: Vec<Self>) -> Vec<Self> {
        let labels_pos: HashMap<String, usize> = selfs
            .iter_mut()
            .enumerate()
            .flat_map(|(line, x)| {
                let o = match x {
                    Self::Label(e) => e.to_owned(),
                    _ => return None,
                };
                *x = Self::Nop;
                Some((o, line))
            })
            .collect();

        selfs.iter_mut().for_each(|x| {
            let o = if let Self::GtLabel(label) = x {
                if let Some(e) = labels_pos.get(label) {
                    *e
                } else {
                    panic!("Invalid label {}", label)
                }
            } else {
                return;
            };
            *x = Self::Gt(o);
        });

        selfs
    }

    pub fn to_raw(&self) -> String {
        match &self {
            Asm::Label(_) => unreachable!(),
            Asm::Op(a, b, c, d) => format!("{} v{}, v{}, v{}", a.to_raw(), b, c, d),
            Asm::Gt(e) => format!("JUMP {}", e),
            Asm::GtLabel(_) => unreachable!(),
            Asm::End => "END".to_owned(),
            Asm::If(a, b, c) => format!("IF{} v{}, v{}", if *a { "N" } else { "" }, b, c),
            Asm::Prt(a) => format!("PRINT v{}", a),
            Asm::Inp(a) => format!("INPUT v{}", a),
            Asm::Cst(a, b) => format!(
                "CONST v{}, {}",
                a,
                b.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Asm::Mov(a, b) => format!("MOV v{}, v{}", a, b),
            Asm::Len(a, b) => format!("LEN v{}, v{}", a, b),
            Asm::Read(a, b, c, d) => format!("READ v{}, v{}, v{}, v{}", a, b, c, d),
            Asm::Nop => format!("NO_OP"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Add,
    And,
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
            OperationType::And => "AND",
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
