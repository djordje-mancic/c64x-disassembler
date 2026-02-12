use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::{
    Unit,
    register::{ControlRegister, Register},
};

pub fn parse(
    opcode: u32,
    format: &[ParsingInstruction],
) -> Result<HashMap<String, ParsedVariable>> {
    let mut resulting_map = HashMap::<String, ParsedVariable>::new();
    let mut temp_opcode = opcode;

    for instruction in format {
        match instruction {
            ParsingInstruction::Match { size, value } => {
                let masked_value = read_u32(&mut temp_opcode, *size);
                if masked_value != *value {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Opcode does not match instruction format (got {masked_value:b} instead of {value:b})"
                        ),
                    ));
                }
            }
            ParsingInstruction::MatchMultiple { size, values } => {
                let masked_value = read_u32(&mut temp_opcode, *size);
                for value in values {
                    if masked_value == *value {
                        continue;
                    }
                }
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Opcode does not match instruction format (got {masked_value:b})"),
                ));
            }
            ParsingInstruction::Bit { name } => {
                resulting_map.insert(
                    name.clone(),
                    ParsedVariable::Bool(read_bool(&mut temp_opcode)),
                );
            }
            ParsingInstruction::BitMatch { name, value } => {
                let read_value = read_bool(&mut temp_opcode);
                if read_value != *value {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Opcode does not match instruction format ({name} is {read_value} instead of {value})"
                        ),
                    ));
                }
                resulting_map.insert(name.clone(), ParsedVariable::Bool(read_value));
            }
            ParsingInstruction::BitArray { size, name } => {
                let mut value = Vec::<bool>::new();
                for _ in 0..*size {
                    value.push(read_bool(&mut temp_opcode));
                }
                resulting_map.insert(name.clone(), ParsedVariable::BoolVec(value));
            }
            ParsingInstruction::Unsigned { size, name } => {
                let value = read_u32(&mut temp_opcode, *size);
                let parsed_variable = {
                    if *size > 8 {
                        ParsedVariable::U32(value)
                    } else {
                        ParsedVariable::U8(value as u8)
                    }
                };
                resulting_map.insert(name.clone(), parsed_variable);
            }
            ParsingInstruction::Register { size, name }
            | ParsingInstruction::RegisterCrosspath { size, name }
            | ParsingInstruction::RegisterPair { size, name } => {
                let u32_value = read_u32(&mut temp_opcode, *size);
                let side = {
                    let mut s = ParsedVariable::try_get(&resulting_map, "s")?.get_bool()?;
                    if let ParsingInstruction::RegisterCrosspath { size: _, name: _ } = instruction
                    {
                        let crosspath = ParsedVariable::try_get(&resulting_map, "x")?.get_bool()?;
                        s ^= crosspath;
                    }
                    s
                };
                let value = {
                    if let ParsingInstruction::RegisterPair { size: _, name: _ } = instruction {
                        Register::from_pair(u32_value as u8, side)
                    } else {
                        Register::from(u32_value as u8, side)
                    }
                };
                resulting_map.insert(name.clone(), ParsedVariable::Register(value));
            }
            ParsingInstruction::ControlRegister { size, name } => {
                let low_bits = read_u32(&mut temp_opcode, *size) as u8;
                let high_bits = {
                    if let Ok(variable) = ParsedVariable::try_get(&resulting_map, "crhi") {
                        variable.get_u8()?
                    } else {
                        0
                    }
                };
                let Some(value) = ControlRegister::from(low_bits, high_bits) else {
                    return Err(Error::other(format!(
                        "Invalid Control Register values (got crhi crlo {high_bits:b} {low_bits:b})"
                    )));
                };
                resulting_map.insert(name.clone(), ParsedVariable::ControlRegister(value));
            }
            ParsingInstruction::LSDUnit { name } => {
                let unit = match read_u32(&mut temp_opcode, 2) {
                    0 => Unit::L,
                    1 => Unit::S,
                    2 => Unit::D,
                    num => return Err(Error::other(format!("Invalid LSDUnit (got {num})"))),
                };
                resulting_map.insert(name.clone(), ParsedVariable::Unit(unit));
            }
        }
    }

    Ok(resulting_map)
}

#[derive(Debug)]
pub enum ParsingInstruction {
    Match {
        size: usize,
        value: u32,
    },
    MatchMultiple {
        size: usize,
        values: Vec<u32>,
    },
    Bit {
        name: String,
    },
    BitMatch {
        name: String,
        value: bool,
    },
    BitArray {
        size: usize,
        name: String,
    },
    Unsigned {
        size: usize,
        name: String,
    },
    Register {
        size: usize,
        name: String,
    },
    RegisterPair {
        size: usize,
        name: String,
    },
    RegisterCrosspath {
        size: usize,
        name: String,
    },
    ControlRegister {
        size: usize,
        name: String,
    },
    /// For .L .S and .D compact maps.
    LSDUnit {
        name: String,
    },
}

#[derive(Clone)]
pub enum ParsedVariable {
    Bool(bool),
    BoolVec(Vec<bool>),
    U32(u32),
    U8(u8),
    Register(Register),
    ControlRegister(ControlRegister),
    Unit(Unit),
}

impl ParsedVariable {
    pub fn get_bool(&self) -> Result<bool> {
        if let ParsedVariable::Bool(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Bool variable"))
        }
    }

    pub fn get_bool_vec(&self) -> Result<Vec<bool>> {
        if let ParsedVariable::BoolVec(value) = self {
            Ok(value.clone())
        } else {
            Err(Error::other("Not a BoolVec variable"))
        }
    }

    pub fn get_u32(&self) -> Result<u32> {
        if let ParsedVariable::U32(value) = self {
            Ok(*value)
        } else if let ParsedVariable::U8(value) = self {
            Ok(*value as u32)
        } else {
            Err(Error::other("Not a U32 variable"))
        }
    }

    pub fn get_u8(&self) -> Result<u8> {
        if let ParsedVariable::U8(value) = self {
            Ok(*value)
        } else if let ParsedVariable::U32(value) = self {
            Ok(*value as u8)
        } else {
            Err(Error::other("Not a U8 variable"))
        }
    }

    pub fn get_register(&self) -> Result<Register> {
        if let ParsedVariable::Register(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Register variable"))
        }
    }

    pub fn get_control_register(&self) -> Result<ControlRegister> {
        if let ParsedVariable::ControlRegister(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Control Register variable"))
        }
    }

    pub fn get_unit(&self) -> Result<Unit> {
        if let ParsedVariable::Unit(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Unit variable"))
        }
    }

    pub fn try_get<'a>(hashmap: &'a HashMap<String, Self>, name: &str) -> Result<&'a Self> {
        let Some(value) = hashmap.get(name) else {
            return Err(Error::other("Parsing error"));
        };
        Ok(value)
    }
}

fn read_bool(opcode: &mut u32) -> bool {
    let value = if *opcode & 1 == 1 { true } else { false };
    *opcode >>= 1;
    value
}

fn read_u32(opcode: &mut u32, size: usize) -> u32 {
    let mask = create_mask(size);
    let value = *opcode & mask;
    *opcode >>= size;
    value
}

fn create_mask(size: usize) -> u32 {
    let mut mask = 0u32;
    for _ in 0..size {
        mask <<= 1;
        mask += 1;
    }
    mask
}
