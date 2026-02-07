use std::io::{Error, ErrorKind, Result};

use crate::instruction::{C64xInstruction, DestinationSide};

pub struct NoUnitInstruction {
    opcode: u32,
    name: String,
    pub parallel: bool,
    pub side: DestinationSide,
    pub csta: Option<u32>,
    pub cstb: Option<u32>,
    compact: bool,
}

impl NoUnitInstruction {
    pub fn new(opcode: u32) -> Result<Self> {
        let mut read_opcode = opcode;
        let mut instruction = NoUnitInstruction::default();
        instruction.opcode = opcode;
        instruction.parallel = if read_opcode & 1 == 1 { true } else { false };
        read_opcode >>= 1;

        if read_opcode & 0xFFF == 0 && (read_opcode << 17) & 0x7FFF == 0 {
            nop_idle(&mut instruction);
            return Ok(instruction);
        }

        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Not an S Unit instruction",
        ));
    }

    pub fn new_compact(opcode: u16) -> Result<Self> {
        let mut instruction = NoUnitInstruction::default();
        instruction.compact = true;
        instruction.opcode = opcode as u32;

        if opcode & 0xFFF == 0xC6E {
            nop_idle(&mut instruction);
            return Ok(instruction);
        }

        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Not an S Unit instruction",
        ));
    }
}

impl C64xInstruction for NoUnitInstruction {
    fn instruction(&self) -> String {
        self.name.clone()
    }

    fn operands(&self) -> String {
        let mut ret_str = String::new();
        if let Some(csta) = self.csta {
            ret_str += format!("0x{csta:04X} ").as_str()
        }
        ret_str
    }

    fn opcode(&self) -> u32 {
        self.opcode
    }

    fn is_compact(&self) -> bool {
        self.compact
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for NoUnitInstruction {
    fn default() -> Self {
        Self {
            opcode: 0,
            name: String::from("UNKNOWN"),
            parallel: false,
            side: DestinationSide::A,
            csta: None,
            cstb: None,
            compact: false,
        }
    }
}

/// Works for both compact and regular instructions
fn nop_idle(instruction: &mut NoUnitInstruction) {
    let src = (instruction.opcode >> 13) & 0b1111;
    if src == 0b1111 {
        instruction.name = String::from("IDLE");
    } else {
        instruction.name = String::from("NOP");
        if src > 0 {
            instruction.csta = Some(src + 1);
        }
    }
}
