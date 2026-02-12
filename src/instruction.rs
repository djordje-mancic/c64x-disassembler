use crate::instruction::register::Register;
use std::any::Any;
use std::io::{Error, ErrorKind, Result};

pub mod fphead;
pub mod invalid;
pub mod moving;
pub mod nop;
pub mod parser;
pub mod register;

pub trait C64xInstruction {
    fn new(_opcode: u32) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::new(ErrorKind::Unsupported, "Instruction not 32-bit"))
    }
    fn new_compact(_opcode: u16) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Instruction not compact (16-bit)",
        ))
    }
    fn instruction(&self) -> String;
    fn instruction_clean(&self) -> String {
        self.instruction()
    }
    fn operands(&self) -> String {
        String::from("")
    }
    fn opcode(&self) -> u32;
    fn is_compact(&self) -> bool {
        false
    }
    fn is_parallel(&self) -> bool {
        false
    }
    fn conditional_operation(&self) -> Option<ConditionalOperation> {
        None
    }
    fn as_any(&self) -> &dyn Any;
}

#[derive(PartialEq, Eq)]
pub enum DataSize {
    Byte,
    ByteUnsigned,
    HalfWord,
    HalfWordUnsigned,
    Word,
    NonAlignedWord,
    DoubleWord,
    NonAlignedDoubleWord,
}

impl DataSize {
    fn to_short_string(&self) -> String {
        match self {
            Self::Byte => String::from("B"),
            Self::ByteUnsigned => String::from("BU"),
            Self::HalfWord => String::from("H"),
            Self::HalfWordUnsigned => String::from("HU"),
            Self::Word => String::from("W"),
            Self::NonAlignedWord => String::from("NW"),
            Self::DoubleWord => String::from("DW"),
            Self::NonAlignedDoubleWord => String::from("NDW"),
        }
    }
}

impl ToString for DataSize {
    fn to_string(&self) -> String {
        match self {
            Self::Byte => String::from("Byte"),
            Self::ByteUnsigned => String::from("ByteUnsigned"),
            Self::HalfWord => String::from("HalfWord"),
            Self::HalfWordUnsigned => String::from("HalfWordUnsigned"),
            Self::Word => String::from("Word"),
            Self::NonAlignedWord => String::from("NonAlignedWord"),
            Self::DoubleWord => String::from("DoubleWord"),
            Self::NonAlignedDoubleWord => String::from("NonAlignedDoubleWord"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Unit {
    L,
    S,
    M,
    D,
}

impl Unit {
    pub fn to_sided_string(&self, side: bool) -> String {
        let mut value = self.to_string();
        if side == false {
            value += "1";
        } else {
            value += "2";
        }
        value
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Self::L => String::from("L"),
            Self::S => String::from("S"),
            Self::M => String::from("M"),
            Self::D => String::from("D"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConditionalOperation {
    Zero(Register),
    NonZero(Register),
}

impl ConditionalOperation {
    pub fn from(creg: u8, z: bool) -> Option<Self> {
        let register_option = {
            if creg & 0b100 == 0b100 {
                match creg & 0b11 {
                    0b00 => Some(Register::A(1)),
                    0b01 => Some(Register::A(2)),
                    0b10 => Some(Register::A(0)),
                    _ => None,
                }
            } else {
                match creg & 0b11 {
                    0b01 => Some(Register::B(0)),
                    0b10 => Some(Register::B(1)),
                    0b11 => Some(Register::B(2)),
                    _ => None,
                }
            }
        };

        if let Some(register) = register_option {
            if z {
                Some(ConditionalOperation::Zero(register))
            } else {
                Some(ConditionalOperation::NonZero(register))
            }
        } else {
            None
        }
    }
}

impl ToString for ConditionalOperation {
    fn to_string(&self) -> String {
        match self {
            ConditionalOperation::NonZero(register) => register.to_string(),
            ConditionalOperation::Zero(register) => format!("!{}", register.to_string()),
        }
    }
}
