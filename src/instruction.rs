use std::any::Any;
use std::io::{Error, ErrorKind, Result};

pub mod fphead;
pub mod invalid;
pub mod moving;
pub mod nop;
pub mod parser;

pub trait C64xInstruction {
    fn new(opcode: u32) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::new(ErrorKind::Unsupported, "Instruction not 32-bit"))
    }
    fn new_compact(opcode: u16) -> Result<Self>
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
            DataSize::Byte => String::from("B"),
            DataSize::ByteUnsigned => String::from("BU"),
            DataSize::HalfWord => String::from("H"),
            DataSize::HalfWordUnsigned => String::from("HU"),
            DataSize::Word => String::from("W"),
            DataSize::NonAlignedWord => String::from("NW"),
            DataSize::DoubleWord => String::from("DW"),
            DataSize::NonAlignedDoubleWord => String::from("NDW"),
        }
    }
}

impl ToString for DataSize {
    fn to_string(&self) -> String {
        match self {
            DataSize::Byte => String::from("Byte"),
            DataSize::ByteUnsigned => String::from("ByteUnsigned"),
            DataSize::HalfWord => String::from("HalfWord"),
            DataSize::HalfWordUnsigned => String::from("HalfWordUnsigned"),
            DataSize::Word => String::from("Word"),
            DataSize::NonAlignedWord => String::from("NonAlignedWord"),
            DataSize::DoubleWord => String::from("DoubleWord"),
            DataSize::NonAlignedDoubleWord => String::from("NonAlignedDoubleWord"),
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
            Unit::L => String::from("L"),
            Unit::S => String::from("S"),
            Unit::M => String::from("M"),
            Unit::D => String::from("D"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Register {
    A(u8),
    B(u8),
}

impl Register {
    pub fn from(value: u8, side: bool) -> Self {
        if side == false {
            Register::A(value)
        } else {
            Register::B(value)
        }
    }

    pub fn side(&self) -> bool {
        match self {
            Register::A(_) => false,
            Register::B(_) => true,
        }
    }
}

impl ToString for Register {
    fn to_string(&self) -> String {
        match self {
            Register::A(num) => return String::from("A") + num.to_string().as_str(),
            Register::B(num) => return String::from("B") + num.to_string().as_str(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConditionalOperation {
    Zero(Register),
    NonZero(Register),
}

impl ConditionalOperation {
    pub fn from(creg: u32, z: bool) -> Option<Self> {
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
