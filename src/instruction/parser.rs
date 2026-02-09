use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::Register;

pub fn parse(
    opcode: u32,
    format: &[ParsingInstruction],
) -> Result<HashMap<String, ParsedVariable>> {
    let mut resulting_map = HashMap::<String, ParsedVariable>::new();
    let mut temp_opcode = opcode;

    for instruction in format {
        match instruction {
            ParsingInstruction::Match { size, value } => {
                let mask = create_mask(*size);
                let masked_value = temp_opcode & mask;
                if masked_value != *value {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Opcode does not match instruction format (got {masked_value:b} instead of {value:b})"
                        ),
                    ));
                }
                temp_opcode >>= size;
            }
            ParsingInstruction::Bit { name } => {
                let value = read_bool(temp_opcode);
                resulting_map.insert(name.clone(), ParsedVariable::Bool(value));
                temp_opcode >>= 1;
            }
            ParsingInstruction::BitArray { size, name } => {
                let mut value = Vec::<bool>::new();
                for _ in 0..*size {
                    value.push(read_bool(temp_opcode));
                    temp_opcode >>= 1;
                }
                resulting_map.insert(name.clone(), ParsedVariable::BoolVec(value));
            }
            ParsingInstruction::Unsigned { size, name } => {
                let mask = create_mask(*size);
                let value = temp_opcode & mask;
                temp_opcode >>= size;
                resulting_map.insert(name.clone(), ParsedVariable::U32(value));
            }
            ParsingInstruction::Register { size, name } => {
                let mask = create_mask(*size);
                let u32_value = temp_opcode & mask;
                let side = ParsedVariable::try_get(&resulting_map, "s")?.get_bool()?;
                let value = Register::from(u32_value as u8, side);
                temp_opcode >>= size;
                resulting_map.insert(name.clone(), ParsedVariable::Register(value));
            }
        }
    }

    Ok(resulting_map)
}

#[derive(Debug)]
pub enum ParsingInstruction {
    Match { size: usize, value: u32 },
    Bit { name: String },
    BitArray { size: usize, name: String },
    Unsigned { size: usize, name: String },
    Register { size: usize, name: String },
}

#[derive(Clone)]
pub enum ParsedVariable {
    Bool(bool),
    BoolVec(Vec<bool>),
    U32(u32),
    Register(Register),
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
        } else {
            Err(Error::other("Not a U32 variable"))
        }
    }

    pub fn get_register(&self) -> Result<Register> {
        if let ParsedVariable::Register(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Register variable"))
        }
    }

    pub fn try_get<'a>(hashmap: &'a HashMap<String, Self>, name: &str) -> Result<&'a Self> {
        let Some(value) = hashmap.get(name) else {
            return Err(Error::other("Parsing error"));
        };
        Ok(value)
    }
}

fn read_bool(opcode: u32) -> bool {
    if opcode & 1 == 1 { true } else { false }
}

fn create_mask(size: usize) -> u32 {
    let mut mask = 0u32;
    for _ in 0..size {
        mask <<= 1;
        mask += 1;
    }
    mask
}
