use crate::instruction::{
    C64xInstruction, ConditionalOperation, Register, Unit,
    parser::{ParsedVariable, ParsingInstruction, parse},
};

pub struct MoveConstantInstruction {
    opcode: u32,
    pub parallel: bool,
    pub high: bool,
    pub constant: u32,
    pub destination: Register,
    compact: bool,
    pub unit: Unit,
    conditional_operation: Option<ConditionalOperation>,
}

impl C64xInstruction for MoveConstantInstruction {
    fn new(opcode: u32) -> std::io::Result<Self> {
        let format_combinations = [
            (
                Unit::S,
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("p"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 4,
                        value: 0b01010,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("h"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 16,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("z"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("creg"),
                    },
                ],
            ),
            (
                Unit::L,
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("p"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 10,
                        value: 0b0011010110,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("x"),
                    },
                    ParsingInstruction::Match {
                        size: 5,
                        value: 0b00101,
                    },
                    ParsingInstruction::Unsigned {
                        size: 5,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("z"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("creg"),
                    },
                ],
            ),
            (
                Unit::D,
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("p"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 11,
                        value: 0b00000010000,
                    },
                    ParsingInstruction::Unsigned {
                        size: 5,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Match { size: 5, value: 0 },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("z"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("creg"),
                    },
                ],
            ),
        ];
        for (unit, format) in format_combinations {
            let Ok(parsed_variables) = parse(opcode, format.as_slice()) else {
                continue;
            };
            let next_parallel = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
            let parallel = false;
            let constant = ParsedVariable::try_get(&parsed_variables, "cst")?.get_u32()?;
            let destination = ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            let high = {
                if unit == Unit::S {
                    ParsedVariable::try_get(&parsed_variables, "h")?.get_bool()?
                } else {
                    false
                }
            };
            let conditional_operation = {
                let creg = ParsedVariable::try_get(&parsed_variables, "creg")?.get_u32()?;
                let z = ParsedVariable::try_get(&parsed_variables, "z")?.get_bool()?;
                ConditionalOperation::from(creg, z)
            };
            return Ok(Self {
                opcode,
                parallel,
                high,
                constant,
                destination,
                compact: false,
                unit,
                conditional_operation,
            });
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Constant instruction: No matches found."),
        ))
    }

    fn new_compact(opcode: u16) -> std::io::Result<Self> {
        let format_combinations = [
            (
                Unit::S,
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 4,
                        value: 0b1001,
                    },
                    ParsingInstruction::Unsigned {
                        size: 2,
                        name: String::from("cst65"),
                    },
                    ParsingInstruction::Register {
                        size: 3,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 1,
                        name: String::from("cst7"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 2,
                        name: String::from("cst43"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("cst20"),
                    },
                ],
            ),
            (
                Unit::L,
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 6,
                        value: 0b010011,
                    },
                    ParsingInstruction::Register {
                        size: 3,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::Match { size: 1, value: 1 },
                    ParsingInstruction::Unsigned {
                        size: 2,
                        name: String::from("cst43"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("cst20"),
                    },
                ],
            ),
        ];
        for (unit, format) in format_combinations {
            let Ok(parsed_variables) = parse(opcode as u32, format.as_slice()) else {
                continue;
            };
            let mut constant = ParsedVariable::try_get(&parsed_variables, "cst20")?.get_u32()?;
            constant += ParsedVariable::try_get(&parsed_variables, "cst43")?.get_u32()? << 3;
            if unit == Unit::S {
                constant += ParsedVariable::try_get(&parsed_variables, "cst65")?.get_u32()? << 5;
                constant += ParsedVariable::try_get(&parsed_variables, "cst7")?.get_u32()? << 7;
            }
            let destination = ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            return Ok(Self {
                opcode: opcode as u32,
                parallel: false,
                high: false,
                constant,
                destination,
                compact: true,
                unit,
                conditional_operation: None,
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Constant instruction: No matches found."),
        ))
    }

    fn instruction_clean(&self) -> String {
        if self.high {
            String::from("MVKH")
        } else {
            String::from("MVK")
        }
    }

    fn instruction(&self) -> String {
        let mut value = self.instruction_clean();
        value += ".";
        let side = self.destination.side();
        value += self.unit.to_sided_string(side).as_str();
        value
    }

    fn opcode(&self) -> u32 {
        self.opcode
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn operands(&self) -> String {
        format!("0x{:04X}, {}", self.constant, self.destination.to_string())
    }

    fn is_compact(&self) -> bool {
        self.compact
    }

    fn is_parallel(&self) -> bool {
        self.parallel
    }

    fn conditional_operation(&self) -> Option<ConditionalOperation> {
        self.conditional_operation
    }
}
