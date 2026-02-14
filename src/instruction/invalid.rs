use crate::instruction::{C64xInstruction, InstructionData};

pub struct InvalidInstruction {
    instruction_data: InstructionData,
}

impl C64xInstruction for InvalidInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        Ok(InvalidInstruction {
            instruction_data: InstructionData {
                opcode: input.opcode,
                ..Default::default()
            },
        })
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        Ok(InvalidInstruction {
            instruction_data: InstructionData {
                opcode: input.opcode,
                compact: true,
                ..Default::default()
            },
        })
    }

    fn instruction(&self) -> String {
        if self.is_compact() {
            String::from("INVALID COMPACT INSTRUCTION")
        } else {
            String::from("INVALID INSTRUCTION")
        }
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
