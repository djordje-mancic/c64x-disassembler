use std::io::{Error, ErrorKind, Result};

use crate::instruction::{
    C64xInstruction, DataSize,
    parser::{ParsedVariable, ParsingInstruction, parse},
};

#[derive(Clone)]
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
    pub parallel_instructions: [bool; 14],
}

impl C64xInstruction for CompactInstructionHeader {
    fn new(input: &super::InstructionInput) -> Result<Self> {
        let format = [
            ParsingInstruction::BitArray {
                size: 14,
                name: String::from("p"),
            },
            ParsingInstruction::Bit {
                name: String::from("SAT"),
            },
            ParsingInstruction::Bit {
                name: String::from("BR"),
            },
            ParsingInstruction::Unsigned {
                size: 2,
                name: String::from("DSZ_1"),
            },
            ParsingInstruction::Bit {
                name: String::from("DSZ_2"),
            },
            ParsingInstruction::Bit {
                name: String::from("RS"),
            },
            ParsingInstruction::Bit {
                name: String::from("PROT"),
            },
            ParsingInstruction::BitArray {
                size: 7,
                name: String::from("layout"),
            },
            ParsingInstruction::Match {
                size: 4,
                value: 0b1110,
            },
        ];
        let parsed_variables = parse(input.opcode, &format).map_err(|e| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("Not a compact instruction header: {e}"),
            )
        })?;

        let layout = {
            let layout_vec =
                ParsedVariable::try_get(&parsed_variables, "layout")?.get_bool_vec()?;
            let Some(layout_ref) = layout_vec.first_chunk::<7>() else {
                return Err(Error::other("Layout doesn't have 7 elements"));
            };
            *layout_ref
        };
        let parallel_instructions = {
            let layout_vec = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool_vec()?;
            let Some(layout_ref) = layout_vec.first_chunk::<14>() else {
                return Err(Error::other("P-bits don't hhave 14 elements"));
            };
            *layout_ref
        };
        let loads_protected = ParsedVariable::try_get(&parsed_variables, "PROT")?.get_bool()?;
        let register_set = ParsedVariable::try_get(&parsed_variables, "RS")?.get_bool()?;
        let data_sizes_1 = ParsedVariable::try_get(&parsed_variables, "DSZ_1")?.get_u8()?;
        let data_sizes_2 = ParsedVariable::try_get(&parsed_variables, "DSZ_2")?.get_bool()?;
        let primary_data_size = {
            if data_sizes_2 == true {
                DataSize::DoubleWord
            } else {
                DataSize::Word
            }
        };
        let secondary_data_size = {
            if primary_data_size == DataSize::DoubleWord {
                match data_sizes_1 {
                    0 => DataSize::Word,
                    1 => DataSize::Byte,
                    2 => DataSize::NonAlignedWord,
                    3 => DataSize::HalfWord,
                    _ => DataSize::Word,
                }
            } else {
                match data_sizes_1 {
                    0 => DataSize::ByteUnsigned,
                    1 => DataSize::Byte,
                    2 => DataSize::HalfWordUnsigned,
                    3 => DataSize::HalfWord,
                    _ => DataSize::ByteUnsigned,
                }
            }
        };
        let decode_compact_branches =
            ParsedVariable::try_get(&parsed_variables, "BR")?.get_bool()?;
        let saturate = ParsedVariable::try_get(&parsed_variables, "SAT")?.get_bool()?;
        Ok(Self {
            opcode: input.opcode,
            layout,
            parallel_instructions,
            loads_protected,
            register_set,
            primary_data_size,
            secondary_data_size,
            decode_compact_branches,
            saturate,
        })
    }

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
}
