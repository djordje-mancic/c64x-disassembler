use crate::instruction::{
    C64xInstruction, InstructionData,
    parser::{ParsedVariable, ParsingInstruction, parse},
};
use std::io::{Error, ErrorKind, Result};

pub struct NOPInstruction {
    pub count: u8,
    instruction_data: InstructionData,
}

impl C64xInstruction for NOPInstruction {
    fn new(input: &super::InstructionInput) -> Result<Self> {
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
        let parsed_variables = parse(input.opcode, &format)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Not a NOP/IDLE: {e}")))?;
        let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
        let count = ParsedVariable::try_get(&parsed_variables, "src")?.get_u8()?;
        Ok(NOPInstruction {
            count,
            instruction_data: InstructionData {
                opcode: input.opcode,
                p_bit,
                ..Default::default()
            },
        })
    }

    fn new_compact(input: &super::InstructionInput) -> Result<Self> {
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
        let parsed_variables = parse(input.opcode, &format)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Not a NOP/IDLE: {e}")))?;
        let count = ParsedVariable::try_get(&parsed_variables, "N3")?.get_u8()?;
        Ok(NOPInstruction {
            count,
            instruction_data: InstructionData {
                opcode: input.opcode,
                compact: true,
                ..Default::default()
            },
        })
    }

    fn instruction(&self) -> String {
        if self.count == 0b1111 {
            String::from("IDLE")
        } else {
            String::from("NOP")
        }
    }

    fn operands(&self) -> String {
        if self.count > 0 && self.count != 0b1111 {
            format!("{}", self.count + 1)
        } else {
            String::new()
        }
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
