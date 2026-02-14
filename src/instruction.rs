use crate::instruction::fphead::CompactInstructionHeader;
use crate::instruction::register::Register;
use std::any::Any;
use std::io::{Error, ErrorKind, Result};

pub mod branching;
pub mod fphead;
pub mod invalid;
pub mod moving;
pub mod nop;
pub mod parser;
pub mod register;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait C64xInstruction: AsAny {
    fn new(_input: &InstructionInput) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::new(ErrorKind::Unsupported, "Instruction not 32-bit"))
    }
    fn new_compact(_input: &InstructionInput) -> Result<Self>
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
    fn instruction_data(&self) -> &InstructionData;
    fn instruction_data_mut(&mut self) -> &mut InstructionData;
    fn opcode(&self) -> u32 {
        self.instruction_data().opcode
    }
    fn is_compact(&self) -> bool {
        self.instruction_data().compact
    }
    fn is_parallel(&self) -> bool {
        self.instruction_data().parallel
    }
    fn get_p_bit(&self) -> bool {
        self.instruction_data().p_bit
    }
    fn set_parallel(&mut self, parallel: bool) {
        self.instruction_data_mut().parallel = parallel;
    }
    fn conditional_operation(&self) -> Option<ConditionalOperation> {
        self.instruction_data().conditional_operation
    }
}

pub struct InstructionInput {
    pub opcode: u32,
    pub fphead: Option<CompactInstructionHeader>,
}

impl InstructionInput {
    pub fn new(opcode: u32) -> Self {
        Self {
            opcode,
            fphead: None,
        }
    }
}

#[derive(Clone)]
pub struct InstructionData {
    pub opcode: u32,
    pub compact: bool,
    pub parallel: bool,
    /// Determines if the next instruction will be executed in parallel
    pub p_bit: bool,
    pub conditional_operation: Option<ConditionalOperation>,
}

impl Default for InstructionData {
    fn default() -> Self {
        Self {
            opcode: 0,
            compact: false,
            parallel: false,
            p_bit: false,
            conditional_operation: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
    ReservedLow,
    ReservedHigh,
    Zero(Register),
    NonZero(Register),
}

impl ConditionalOperation {
    pub fn from(creg: u8, z: bool) -> Option<Self> {
        if creg == 0 && z == true {
            return Some(ConditionalOperation::ReservedLow);
        } else if creg == 0b111 {
            return Some(ConditionalOperation::ReservedHigh);
        }
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
            ConditionalOperation::ReservedLow | ConditionalOperation::ReservedHigh => String::new(),
        }
    }
}
