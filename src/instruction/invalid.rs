use std::any::Any;

use crate::instruction::C64xInstruction;

pub struct InvalidInstruction {
    opcode: u32,
    compact: bool,
}

impl C64xInstruction for InvalidInstruction {
    fn new(
        opcode: u32,
        _fphead: Option<&super::fphead::CompactInstructionHeader>,
    ) -> std::io::Result<Self> {
        Ok(InvalidInstruction {
            opcode,
            compact: false,
        })
    }

    fn new_compact(
        opcode: u16,
        _fphead: &super::fphead::CompactInstructionHeader,
    ) -> std::io::Result<Self> {
        Ok(InvalidInstruction {
            opcode: opcode as u32,
            compact: true,
        })
    }

    fn instruction(&self) -> String {
        if self.compact {
            String::from("INVALID COMPACT INSTRUCTION")
        } else {
            String::from("INVALID INSTRUCTION")
        }
    }
    fn opcode(&self) -> u32 {
        self.opcode
    }
    fn is_compact(&self) -> bool {
        self.compact
    }
}
