use std::convert::TryInto;

use crate::asm::Asm;

mod executor;
pub use executor::*;

const CVM_VERSION: u32 = 1;

// u32 (CVM specification version)
// u8 (line_encode_length | var_encode_length) (number of byte per line | number of byte per var) (maximum u64 supported so 4 the u8 is split in two)
// u32 number of instructions
// FOR EACH:
//   u1 is operation
//     if true:
//       u7 operation type
//       UVAR
//       UVAR
//       UVAR
//     else
//       u7 opcode
//       if opcode == 0:
//
//   u8 (1 bytes is equivalent to label) (4 first bits are the OPCODE) (1 other bits are unused)
//   If the opcode is 0x0B (Operation), The next one is for operation discrimination
pub fn clean_asm_to_exe(asm: &Vec<Asm>) -> Vec<u8> {
    let op = match asm.len() {
        0..=255 => 0,
        256..=0xffff => 1,
        _ => 2,
    };
    let config = Config {
        line_encode_length: op,
        var_encode_length: op,
        version: CVM_VERSION,
        number_of_instructions: asm.len() as u32,
    };
    let mut a = Vec::new();
    a.append(&mut config.create_header());
    a.append(&mut asm.iter().map(|x| to_bytes(x, &config)).flatten().collect());
    a
}

struct Config {
    line_encode_length: u8,
    var_encode_length: u8,
    number_of_instructions: u32,
    version: u32,
}

impl Config {
    fn get_header<T>(data: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let version = u32::from_le_bytes(data.take(4).collect::<Vec<u8>>().try_into().ok()?) as u32;
        let read = data.next().unwrap();
        let line_encode_length = (read & 0xF0) >> 4;
        let var_encode_length = read & 0x0F;
        let number_of_instructions =
            u32::from_le_bytes(data.take(4).collect::<Vec<u8>>().try_into().ok()?) as u32;
        Some(Self {
            line_encode_length,
            var_encode_length,
            number_of_instructions,
            version,
        })
    }

    fn create_header(&self) -> Vec<u8> {
        let mut headers = Vec::new();
        headers.append(&mut self.version.to_le_bytes().to_vec());
        headers.push((self.line_encode_length << 4) | self.var_encode_length);
        headers.append(&mut self.number_of_instructions.to_le_bytes().to_vec());
        headers
    }

    fn decode_line<T>(&self, data: &mut T) -> Option<u32>
    where
        T: Iterator<Item = u8>,
    {
        match self.line_encode_length {
            0 => data.next().map(|x| x as u32),
            1 => {
                Some(u16::from_le_bytes(data.take(2).collect::<Vec<u8>>().try_into().ok()?) as u32)
            }
            2 => {
                Some(u32::from_le_bytes(data.take(4).collect::<Vec<u8>>().try_into().ok()?) as u32)
            }
            _ => unreachable!(),
        }
    }
    fn encode_line(&self, line: u32) -> Vec<u8> {
        match self.line_encode_length {
            0 => vec![line as u8],
            1 => (line as u16).to_le_bytes().to_vec(),
            2 => (line as u32).to_le_bytes().to_vec(),
            _ => unreachable!(),
        }
    }
    fn encode_var(&self, var: u32) -> Vec<u8> {
        match self.var_encode_length {
            0 => vec![var as u8],
            1 => (var as u16).to_le_bytes().to_vec(),
            2 => (var as u32).to_le_bytes().to_vec(),
            _ => unreachable!(),
        }
    }
    fn decode_var<T>(&self, data: &mut T) -> Option<u32>
    where
        T: Iterator<Item = u8>,
    {
        match self.var_encode_length {
            0 => data.next().map(|x| x as u32),
            1 => {
                Some(u16::from_le_bytes(data.take(2).collect::<Vec<u8>>().try_into().ok()?) as u32)
            }
            2 => {
                Some(u32::from_le_bytes(data.take(4).collect::<Vec<u8>>().try_into().ok()?) as u32)
            }
            _ => unreachable!(),
        }
    }
}

use crate::asm::OperationType;

pub fn exe_to_clean_asm(vec: Vec<u8>) -> Option<Vec<Asm>> {
    let mut iter = vec.into_iter();
    let header = Config::get_header(&mut iter).unwrap();
    let mut out = Vec::new();
    while let Some(e) = from_bytes(&mut iter, &header) {
        out.push(e);
    }
    Some(out)
}

fn from_bytes<T>(data: &mut T, config: &Config) -> Option<Asm>
where
    T: Iterator<Item = u8>,
{
    let i = data.next()?;
    Some(match i {
        128..=255 => Asm::Op(
            match i {
                129 => OperationType::Add,
                130 => OperationType::And,
                131 => OperationType::Sub,
                132 => OperationType::Mul,
                133 => OperationType::Div,
                134 => OperationType::Mod,
                135 => OperationType::Xor,
                136 => OperationType::Or,
                137 => OperationType::Merge,
                _ => return None,
            },
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
        ),
        0 => Asm::Nop,
        1 => Asm::Read(
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
        ),
        2 => Asm::Len(
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
        ),
        3 => Asm::Mov(
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
        ),
        4 => {
            let i = config.decode_var(data)? as usize;
            let len = data.next()?;
            Asm::Cst(i, data.take(len as usize).collect())
        }
        5 => Asm::Inp(config.decode_var(data)? as usize),
        6 => Asm::Prt(config.decode_var(data)? as usize),
        7 => Asm::If(
            false,
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
        ),
        8 => Asm::If(
            true,
            config.decode_var(data)? as usize,
            config.decode_var(data)? as usize,
        ),
        9 => Asm::End,
        10 => Asm::Gt(config.decode_line(data)? as usize),
        _ => return None,
    })
}

fn to_bytes(asm: &Asm, config: &Config) -> Vec<u8> {
    let mut vec = Vec::new();
    match asm {
        Asm::Label(_) => unreachable!(),
        Asm::Op(a, b, c, d) => {
            vec.push(
                128 + match a {
                    OperationType::Add => 1,
                    OperationType::And => 2,
                    OperationType::Sub => 3,
                    OperationType::Mul => 4,
                    OperationType::Div => 5,
                    OperationType::Mod => 6,
                    OperationType::Xor => 7,
                    OperationType::Or => 8,
                    OperationType::Merge => 9,
                },
            );
            vec.append(&mut config.encode_var(*b as u32));
            vec.append(&mut config.encode_var(*c as u32));
            vec.append(&mut config.encode_var(*d as u32));
        }
        Asm::Gt(e) => {
            vec.push(10);
            vec.append(&mut config.encode_line(*e as u32));
        }
        Asm::GtLabel(_) => unreachable!(),
        Asm::End => {
            vec.push(9);
        }
        Asm::If(a, b, c) => {
            if *a {
                vec.push(8);
            } else {
                vec.push(7);
            }
            vec.append(&mut config.encode_var(*b as u32));
            vec.append(&mut config.encode_var(*c as u32));
        }
        Asm::Prt(a) => {
            vec.push(6);
            vec.append(&mut config.encode_var(*a as u32));
        }
        Asm::Inp(a) => {
            vec.push(5);
            vec.append(&mut config.encode_var(*a as u32));
        }
        Asm::Cst(a, b) => {
            vec.push(4);
            vec.append(&mut config.encode_var(*a as u32));
            vec.push(b.len() as u8);
            vec.append(&mut b.clone());
        }
        Asm::Mov(a, b) => {
            vec.push(3);
            vec.append(&mut config.encode_var(*a as u32));
            vec.append(&mut config.encode_var(*b as u32));
        }
        Asm::Len(a, b) => {
            vec.push(2);
            vec.append(&mut config.encode_var(*a as u32));
            vec.append(&mut config.encode_var(*b as u32));
        }
        Asm::Read(a, b, c, d) => {
            vec.push(1);
            vec.append(&mut config.encode_var(*a as u32));
            vec.append(&mut config.encode_var(*b as u32));
            vec.append(&mut config.encode_var(*c as u32));
            vec.append(&mut config.encode_var(*d as u32));
        }
        Asm::Nop => {
            vec.push(0);
        }
    }
    vec
}
