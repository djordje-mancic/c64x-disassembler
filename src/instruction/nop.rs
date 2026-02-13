use crate::instruction::{
    C64xInstruction,
    parser::{ParsedVariable, ParsingInstruction, parse},
};
use std::io::{Error, ErrorKind, Result};

pub struct NOPInstruction {
    pub opcode: u32,
    pub parallel: bool,
    pub count: u8,
    compact: bool,
}

impl C64xInstruction for NOPInstruction {
    fn new(opcode: u32, _fphead: Option<&super::fphead::CompactInstructionHeader>) -> Result<Self> {
        let format = [
            ParsingInstruction::Bit {
                name: String::from("p"),
            },
            ParsingInstruction::Match { size: 12, value: 0 },
            ParsingInstruction::Unsigned {
                size: 4,
                name: String::from("src"),
            },
            ParsingInstruction::Match { size: 15, value: 0 },
        ];
        let parsed_variables = parse(opcode, &format)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Not a NOP/IDLE: {e}")))?;
        let next_parallel = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
        let parallel = false;
        let count = ParsedVariable::try_get(&parsed_variables, "src")?.get_u8()?;
        Ok(NOPInstruction {
            opcode,
            parallel,
            count,
            compact: false,
        })
    }

    fn new_compact(opcode: u16, _fphead: &super::fphead::CompactInstructionHeader) -> Result<Self> {
        let format = [
            ParsingInstruction::Match {
                size: 13,
                value: 0xC6E,
            },
            ParsingInstruction::Unsigned {
                size: 3,
                name: String::from("N3"),
            },
        ];
        let parsed_variables = parse(opcode as u32, &format)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Not a NOP/IDLE: {e}")))?;
        let count = ParsedVariable::try_get(&parsed_variables, "N3")?.get_u8()?;
        Ok(NOPInstruction {
            opcode: opcode as u32,
            parallel: false,
            count,
            compact: true,
        })
    }

    fn instruction(&self) -> String {
        if self.count == 0b1111 {
            String::from("IDLE")
        } else {
            String::from("NOP")
        }
    }

    fn opcode(&self) -> u32 {
        self.opcode
    }

    fn operands(&self) -> String {
        if self.count > 0 && self.count != 0b1111 {
            format!("{}", self.count + 1)
        } else {
            String::new()
        }
    }

    fn is_compact(&self) -> bool {
        self.compact
    }

    fn is_parallel(&self) -> bool {
        self.parallel
    }
}
