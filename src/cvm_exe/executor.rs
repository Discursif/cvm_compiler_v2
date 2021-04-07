use std::{fmt::Display, io::Write};

use crate::asm::{Asm, OperationType};

pub enum CVMRuntimeError {
    RegisterNotFound(usize),
    RegisterEmpty(usize),
    Other(Box<dyn std::error::Error>),
}

impl Display for CVMRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CVMRuntimeError::RegisterNotFound(e) => {
                write!(f, "E8-001: Register {} doesn't exists", e)
            }
            CVMRuntimeError::RegisterEmpty(e) => {
                write!(f, "E8-002: Register {} is empty", e)
            }
            CVMRuntimeError::Other(e) => {
                write!(f, "E8-003: Internal error {}", e)
            }
        }
    }
}

pub struct CVm<'a> {
    instructions: Vec<Asm>,
    caret: usize,
    registers: Vec<Vec<u8>>,
    finished: bool,
    len: usize,
    printer: &'a dyn Fn(&[u8]) -> Result<(), CVMRuntimeError>,
}

impl<'a> CVm<'a> {
    pub fn from_instructions(instructions: Vec<Asm>) -> Self {
        Self {
            printer: &|s| {
                let mut out = std::io::stdout();
                out.write_all(s)
                    .map_err(|x| CVMRuntimeError::Other(box x))?;
                out.flush().map_err(|x| CVMRuntimeError::Other(box x))?;
                Ok(())
            },
            instructions,
            caret: 0,
            registers: Vec::new(),
            finished: false,
            len: 0,
        }
    }

    pub fn set_printer<T>(&mut self, printer: &'a T)
    where
        T: Fn(&[u8]) -> Result<(), CVMRuntimeError>,
    {
        self.printer = printer;
    }

    pub fn reset(&mut self) {
        self.caret = 0;
        self.registers = Vec::new();
        self.finished = false;
        self.len = 0;
    }

    pub fn has_finished(&self) -> bool {
        self.finished
    }

    pub fn get_instructions(&self) -> &Vec<Asm> {
        &self.instructions
    }

    pub fn get_instructions_mut(&mut self) -> &mut Vec<Asm> {
        &mut self.instructions
    }

    pub fn get_caret_position(&self) -> usize {
        self.caret
    }

    pub fn set_caret_position(&mut self, position: usize) {
        self.caret = position.min(self.instructions.len());
    }

    pub fn get_registers(&self) -> &Vec<Vec<u8>> {
        &self.registers
    }

    pub fn get_registers_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.registers
    }

    pub fn get_reg(&self, id: usize) -> Result<&Vec<u8>, CVMRuntimeError> {
        self.registers
            .get(id as usize)
            .ok_or(CVMRuntimeError::RegisterNotFound(id))
    }

    pub fn get_firstvalue(&self, id: usize) -> Result<u8, CVMRuntimeError> {
        Ok(*self
            .get_reg(id)?
            .get(0)
            .ok_or(CVMRuntimeError::RegisterEmpty(id))?)
    }

    pub fn set_reg(&mut self, id: usize, value: Vec<u8>) {
        if self.len <= id {
            self.registers
                .extend((0..=id - self.len + 1).map(|_| Vec::new()));
            self.len = self.registers.len() as usize;
        }
        unsafe {
            *self.registers.get_unchecked_mut(id as usize) = value;
        }
    }

    #[allow(mutable_borrow_reservation_conflict)]
    pub fn execute(&mut self) -> Result<bool, CVMRuntimeError> {
        if self.finished {
            return Ok(false);
        }
        Ok(if let Some(e) = self.instructions.get(self.caret) {
            match e {
                Asm::Inp(into) => {
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|e| CVMRuntimeError::Other(box e))?;
                    self.set_reg(*into, input.trim().bytes().collect());
                }
                Asm::Read(into, from, index, length) => {
                    let o = self.get_firstvalue(*index)? as usize;
                    let arr = self.get_reg(*from)?;
                    self.set_reg(
                        *into,
                        arr[o..(o + self.get_firstvalue(*length)? as usize).min(arr.len())]
                            .to_vec(),
                    );
                }
                Asm::If(inv, var1, var2) => {
                    if *inv {
                        if self.get_reg(*var1)? != self.get_reg(*var2)? {
                            self.caret += 1;
                        }
                    } else {
                        if self.get_reg(*var1)? == self.get_reg(*var2)? {
                            self.caret += 1;
                        }
                    }
                }
                Asm::Gt(line) => {
                    self.caret = *line;
                    return Ok(true);
                }
                Asm::Nop => (),
                Asm::Prt(variable) => {
                    (self.printer)(self.get_reg(*variable)?)?;
                }
                Asm::End => {
                    self.finished = true;
                    return Ok(false);
                }
                Asm::Cst(into, bytes) => self.set_reg(*into, bytes.clone()),
                Asm::Op(operation, into, base, operator) => {
                    if operation == &OperationType::Merge {
                        let mut o = self.get_reg(*base)?.clone();
                        o.extend(self.get_reg(*operator)?.iter());
                        self.set_reg(*into, o.into_iter().take(256).collect());
                    } else {
                        self.set_reg(
                            *into,
                            self.get_reg(*base)?
                                .iter()
                                .zip(self.get_reg(*operator)?.iter().cycle())
                                .map(match operation {
                                    OperationType::Sub => |(x, y): (&u8, &u8)| x.wrapping_sub(*y),
                                    OperationType::Add => |(x, y): (&u8, &u8)| x.wrapping_add(*y),
                                    OperationType::Mul => |(x, y): (&u8, &u8)| x.wrapping_mul(*y),
                                    OperationType::Div => |(x, y): (&u8, &u8)| x.wrapping_div(*y),
                                    OperationType::Mod => |(x, y): (&u8, &u8)| x % y,
                                    OperationType::Xor => |(x, y): (&u8, &u8)| x ^ y,
                                    OperationType::And => |(x, y): (&u8, &u8)| x & y,
                                    OperationType::Or => |(x, y): (&u8, &u8)| x | y,
                                    OperationType::Merge => unreachable!(),
                                })
                                .collect(),
                        )
                    }
                }
                Asm::Len(into, from) => self.set_reg(*into, vec![self.get_reg(*from)?.len() as u8]),
                Asm::Mov(into, from) => {
                    self.set_reg(*into, self.get_reg(*from)?.clone());
                } // Asm::Merge { into, var1, var2 } => {
                //     let mut o = self.get_reg(*var1)?.clone();
                //     o.extend(self.get_reg(*var2)?.iter());
                //     self.set_reg(*into, o);
                // }
                // Asm::IfN { var1, var2 } => {
                //     if self.get_reg(*var1)? != self.get_reg(*var2)? {
                //         self.caret += 1;
                //     }
                // }
                Asm::Label(_) => unreachable!(),
                Asm::GtLabel(_) => unreachable!(),
            }
            self.caret += 1;
            true
        } else {
            self.finished = true;
            false
        })
    }
}
