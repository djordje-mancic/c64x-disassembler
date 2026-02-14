use crate::instruction::C64xInstruction;

pub struct InvalidInstruction {
    opcode: u32,
    compact: bool,
}

impl C64xInstruction for InvalidInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        Ok(InvalidInstruction {
            opcode: input.opcode,
            compact: false,
        })
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        Ok(InvalidInstruction {
            opcode: input.opcode,
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
