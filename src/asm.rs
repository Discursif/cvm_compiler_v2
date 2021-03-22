use std::{collections::HashMap, fmt::Display};

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

impl Display for Asm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Op(a, b, c, d) => write!(f,"v{} = v{} {} v{}",b,c,a.as_operator(),d),
            Self::End => write!(f,"end"),
            Self::Prt(e) => write!(f,"print v{}",e),
            Self::Inp(e) => write!(f,"v{} = input",e),
            Self::Cst(e, a) => write!(f,"v{} = {}",e,a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),
            Self::Mov(e, a) => write!(f,"v{} = v{}",e,a),
            Self::Len(e, a) => write!(f,"v{} = len v{}",e,a),
            Self::Read(a, b, c, d) => write!(f,"v{} = v{}[v{} > v{}]",a,b,c,d),
            Self::Nop => write!(f,""),
            Asm::Label(e) => write!(f,"'{}",e),
            Asm::Gt(e) => write!(f,"goto {}",e),
            Asm::GtLabel(e) => write!(f,"goto '{}",e),
            Asm::If(a, b, c) => write!(f,"if {} {}= {}",b,if *a {
                "!"
            } else {
                "="
            },c),
        }
    }
}

impl Asm {
    pub fn clean(mut selfs: Vec<Self>) -> Vec<Self> {
        let mut line = 0;
        let mut labels_pos: HashMap<String, usize> = HashMap::new();
        selfs = selfs
            .into_iter()
            .flat_map(|x| {
                let o = match x {
                    Self::Label(e) => e.to_owned(),
                    e => {
                        line += 1;
                        return Some(e);
                    },
                };
                labels_pos.insert(o, line);
                None
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn as_operator(&self) -> &'static str {
        match self {
            OperationType::And => "&",
            OperationType::Add => "+",
            OperationType::Sub => "-",
            OperationType::Mul => "*",
            OperationType::Div => "/",
            OperationType::Mod => "%",
            OperationType::Xor => "^",
            OperationType::Merge => "~",
        }
    }
}
