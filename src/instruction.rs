use std::any::Any;

pub mod fphead;
pub mod no_unit;
pub mod s_unit;

pub trait C64xInstruction {
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
    fn as_any(&self) -> &dyn Any;
}

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
pub enum DestinationSide {
    A,
    B,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Register {
    A(u8),
    B(u8),
}

impl Register {
    pub fn from_dest(dest: u8, side: DestinationSide) -> Self {
        if side == DestinationSide::A {
            Register::A(dest)
        } else {
            Register::B(dest)
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

pub struct InvalidInstruction {
    opcode: u32,
}

impl InvalidInstruction {
    pub fn new(opcode: u32) -> Self {
        InvalidInstruction { opcode }
    }
}

impl C64xInstruction for InvalidInstruction {
    fn instruction(&self) -> String {
        String::from("INVALID INSTRUCTION")
    }
    fn opcode(&self) -> u32 {
        self.opcode
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InvalidCompactInstruction {
    opcode: u16,
}

impl InvalidCompactInstruction {
    pub fn new(opcode: u16) -> Self {
        InvalidCompactInstruction { opcode }
    }
}

impl C64xInstruction for InvalidCompactInstruction {
    fn instruction(&self) -> String {
        String::from("INVALID COMPACT INSTRUCTION")
    }
    fn opcode(&self) -> u32 {
        self.opcode as u32
    }
    fn is_compact(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
