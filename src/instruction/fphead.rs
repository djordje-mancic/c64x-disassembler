use std::{
    any::Any,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::{C64xInstruction, DataSize};

pub struct CompactInstructionHeader {
    opcode: u32,
    /// Layout field
    /// Determines if the i-th word holds two compact (16-bit) instructions (true)
    /// or a regular, 32-bit instruction (false).
    pub layout: [bool; 7],
    /// PROT field.
    /// Determines if 4 ``NOP`` cycles are added after every LD instruction.
    pub loads_protected: bool,
    /// RS field.
    /// Determines if instructions use high register set for data source
    /// and destination (true) or low register set (false).
    pub register_set: bool,
    pub primary_data_size: DataSize,
    pub secondary_data_size: DataSize,
    /// BR field.
    /// Determines if compact instructions in the S unit are decoded
    /// as branches.
    pub decode_compact_branches: bool,
    /// SAT field.
    /// Determines if instructions are saturated.
    ///
    /// As a result, ``ADD``, ``SUB``, ``SHL``, ``MPY``, ``MPYH``, ``MPYLH`` and ``MPYHL``
    /// instructions are decoded as ``SADD``, ``SUBS``, ``SSHL``, ``SMPY``, ``SMPYH``, ``SMPYLH`` and
    /// ``SMPYHL`` respectively.
    pub saturate: bool,
}

impl CompactInstructionHeader {
    pub fn new(opcode: u32) -> Result<Self> {
        if (opcode >> 28) & 0b1111 != 0b1110 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Not a compact instruction header",
            ));
        }

        let layout = {
            let mut layout_arr = [false; 7];
            let mut layout_bytes = opcode >> 21;
            for i in 0..7 {
                if layout_bytes & 1 == 1 {
                    layout_arr[i] = true
                }
                layout_bytes >>= 1;
            }
            layout_arr
        };

        let mut loads_protected = false;
        let mut register_set = false;
        let mut primary_data_size = DataSize::Word;
        let mut secondary_data_size = DataSize::ByteUnsigned;
        let mut decode_compact_branches = false;
        let mut saturate = false;

        {
            let mut expansion_bytes = opcode >> 14;
            if expansion_bytes & 1 == 1 {
                saturate = true;
            }
            expansion_bytes >>= 1;

            if expansion_bytes & 1 == 1 {
                decode_compact_branches = true;
            }
            expansion_bytes >>= 1;

            if expansion_bytes & 0b111 >= 0b100 {
                primary_data_size = DataSize::DoubleWord;
                secondary_data_size = match expansion_bytes & 0b11 {
                    0 => DataSize::Word,
                    1 => DataSize::Byte,
                    2 => DataSize::NonAlignedWord,
                    3 => DataSize::HalfWord,
                    _ => DataSize::Word,
                }
            } else {
                primary_data_size = DataSize::Word;
                secondary_data_size = match expansion_bytes & 0b11 {
                    0 => DataSize::ByteUnsigned,
                    1 => DataSize::Byte,
                    2 => DataSize::HalfWordUnsigned,
                    3 => DataSize::HalfWord,
                    _ => DataSize::ByteUnsigned,
                }
            }
            expansion_bytes >>= 3;

            if expansion_bytes & 1 == 1 {
                register_set = true;
            }
            expansion_bytes >>= 1;

            if expansion_bytes & 1 == 1 {
                loads_protected = true;
            }
        }

        Ok(Self {
            opcode,
            layout,
            loads_protected,
            register_set,
            primary_data_size,
            secondary_data_size,
            decode_compact_branches,
            saturate,
        })
    }
}

impl C64xInstruction for CompactInstructionHeader {
    fn instruction(&self) -> String {
        String::from(".fphead")
    }
    fn operands(&self) -> String {
        let mut return_str = String::new();
        if self.loads_protected {
            return_str += "p";
        } else {
            return_str += "n";
        }
        return_str += ", ";
        if self.register_set {
            return_str += "h";
        } else {
            return_str += "l";
        }
        return_str += ", ";
        return_str += (self.primary_data_size.to_short_string() + ", ").as_str();
        return_str += (self.secondary_data_size.to_short_string() + ", ").as_str();
        if self.decode_compact_branches {
            return_str += "br";
        } else {
            return_str += "nobr";
        }
        return_str += ", ";
        if self.saturate {
            return_str += "sat";
        } else {
            return_str += "nosat";
        }
        return_str += ", ";
        for i in (0..7).rev() {
            if self.layout[i] {
                return_str += "1";
            } else {
                return_str += "0";
            }
        }
        return_str
    }
    fn opcode(&self) -> u32 {
        self.opcode
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
