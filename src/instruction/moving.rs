use crate::instruction::{
    C64xInstruction, ConditionalOperation, InstructionData, Unit,
    parser::{ParsedVariable, ParsingInstruction, parse},
    register::{ControlRegister, Register, RegisterFile},
};

pub struct MoveConstantInstruction {
    pub high: bool,
    pub constant: u32,
    pub destination: Register,
    pub unit: Unit,
    instruction_data: InstructionData,
}

impl C64xInstruction for MoveConstantInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
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
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
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
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
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
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
                    },
                ],
            ),
        ];
        for (unit, format) in format_combinations {
            let Ok(parsed_variables) = parse(input.opcode, format.as_slice()) else {
                continue;
            };
            let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
            let constant = ParsedVariable::try_get(&parsed_variables, "cst")?.get_u32()?;
            let destination = ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            let high = {
                if unit == Unit::S {
                    ParsedVariable::try_get(&parsed_variables, "h")?.get_bool()?
                } else {
                    false
                }
            };
            let conditional_operation =
                ParsedVariable::try_get(&parsed_variables, "cond")?.get_conditional_operation()?;
            return Ok(Self {
                high,
                constant,
                destination,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Constant instruction: No matches found."),
        ))
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
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
            let Ok(parsed_variables) = parse(input.opcode, format.as_slice()) else {
                continue;
            };
            let mut constant = ParsedVariable::try_get(&parsed_variables, "cst20")?.get_u8()?;
            constant += ParsedVariable::try_get(&parsed_variables, "cst43")?.get_u8()? << 3;
            if unit == Unit::S {
                constant += ParsedVariable::try_get(&parsed_variables, "cst65")?.get_u8()? << 5;
                constant += ParsedVariable::try_get(&parsed_variables, "cst7")?.get_u8()? << 7;
            }
            let destination = ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            return Ok(Self {
                high: false,
                constant: constant as u32,
                destination,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    ..Default::default()
                },
            });
        }

        let multiunit_formats = [
            vec![
                ParsingInstruction::Bit {
                    name: String::from("s"),
                },
                ParsingInstruction::Match {
                    size: 2,
                    value: 0b11,
                },
                ParsingInstruction::LSDUnit {
                    name: String::from("unit"),
                },
                ParsingInstruction::Match {
                    size: 2,
                    value: 0b11,
                },
                ParsingInstruction::Register {
                    size: 3,
                    name: String::from("dst"),
                },
                ParsingInstruction::Match {
                    size: 3,
                    value: 0b010,
                },
                ParsingInstruction::Unsigned {
                    size: 1,
                    name: String::from("cst"),
                },
                ParsingInstruction::Unsigned {
                    size: 2,
                    name: String::from("cc"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("s"),
                },
                ParsingInstruction::Match {
                    size: 2,
                    value: 0b11,
                },
                ParsingInstruction::LSDUnit {
                    name: String::from("unit"),
                },
                ParsingInstruction::Match {
                    size: 2,
                    value: 0b11,
                },
                ParsingInstruction::Register {
                    size: 3,
                    name: String::from("dst"),
                },
                ParsingInstruction::Match {
                    size: 3,
                    value: 0b110,
                },
                ParsingInstruction::Unsigned {
                    size: 1,
                    name: String::from("cst"),
                },
                ParsingInstruction::Match {
                    size: 2,
                    value: 0b00,
                },
            ],
        ];

        for format in multiunit_formats {
            let Ok(parsed_variables) = parse(input.opcode as u32, format.as_slice()) else {
                continue;
            };
            let constant = ParsedVariable::try_get(&parsed_variables, "cst")?.get_u32()?;
            let destination = ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            let unit = ParsedVariable::try_get(&parsed_variables, "unit")?.get_unit()?;
            let conditional_operation = {
                if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "cc") {
                    match variable.get_u8()? {
                        0 => Some(ConditionalOperation::NonZero(Register::A(0))),
                        1 => Some(ConditionalOperation::Zero(Register::A(0))),
                        2 => Some(ConditionalOperation::NonZero(Register::B(0))),
                        3 => Some(ConditionalOperation::Zero(Register::B(0))),
                        _ => None,
                    }
                } else {
                    None
                }
            };
            return Ok(Self {
                high: false,
                constant,
                destination,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    conditional_operation,
                    compact: true,
                    ..Default::default()
                },
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

    fn operands(&self) -> String {
        format!("0x{:04X}, {}", self.constant, self.destination.to_string())
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}

pub struct MoveRegisterInstruction {
    pub source: RegisterFile,
    pub destination: RegisterFile,
    side: bool,
    pub delayed: bool,
    pub unit: Unit,
    instruction_data: InstructionData,
}

impl MoveRegisterInstruction {
    fn new_mv(opcode: u32) -> std::io::Result<Self> {
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
                        size: 10,
                        value: 0b0001101000,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("x"),
                    },
                    ParsingInstruction::Match { size: 5, value: 0 },
                    ParsingInstruction::RegisterCrosspath {
                        size: 5,
                        name: String::from("src"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
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
                        size: 16,
                        value: 0x106,
                    },
                    ParsingInstruction::RegisterPair {
                        size: 5,
                        name: String::from("src"),
                    },
                    ParsingInstruction::RegisterPair {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
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
                        size: 3,
                        value: 0b110,
                    },
                    ParsingInstruction::MatchMultiple {
                        size: 7,
                        values: vec![0x2, 0x7E],
                    },
                    ParsingInstruction::Bit {
                        name: String::from("x"),
                    },
                    ParsingInstruction::Match { size: 5, value: 0 },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("src"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
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
                        size: 16,
                        value: 0x250,
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("src"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
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
                        size: 10,
                        value: 0x23C,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("x"),
                    },
                    ParsingInstruction::Match { size: 5, value: 0 },
                    ParsingInstruction::RegisterCrosspath {
                        size: 5,
                        name: String::from("src"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
                    },
                ],
            ),
            (
                Unit::M,
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("p"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 10,
                        value: 0x3C,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("x"),
                    },
                    ParsingInstruction::Match {
                        size: 5,
                        value: 0x1A,
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("src"),
                    },
                    ParsingInstruction::Register {
                        size: 5,
                        name: String::from("dst"),
                    },
                    ParsingInstruction::ConditionalOperation {
                        name: String::from("cond"),
                    },
                ],
            ),
        ];
        for (unit, format) in format_combinations {
            let Ok(parsed_variables) = parse(opcode, format.as_slice()) else {
                continue;
            };
            let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
            let side = ParsedVariable::try_get(&parsed_variables, "s")?.get_bool()?;
            let source_register =
                ParsedVariable::try_get(&parsed_variables, "src")?.get_register()?;
            let destination_register =
                ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            let source = RegisterFile::GeneralPurpose(source_register);
            let destination = RegisterFile::GeneralPurpose(destination_register);
            let conditional_operation =
                ParsedVariable::try_get(&parsed_variables, "cond")?.get_conditional_operation()?;
            let delayed = if unit == Unit::M { true } else { false };
            return Ok(Self {
                source,
                destination,
                unit,
                side,
                delayed,
                instruction_data: InstructionData {
                    opcode,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }
        Err(std::io::Error::other("Not MV/MVD"))
    }

    fn new_mvc(opcode: u32) -> std::io::Result<Self> {
        let format_combinations = [
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::BitMatch {
                    name: String::from("s"),
                    value: true,
                },
                ParsingInstruction::Match {
                    size: 10,
                    value: 0b0011111000,
                },
                ParsingInstruction::Bit {
                    name: String::from("x"),
                },
                ParsingInstruction::Match { size: 5, value: 0 },
                ParsingInstruction::ControlRegister {
                    size: 5,
                    name: String::from("crlo"),
                },
                ParsingInstruction::Register {
                    size: 5,
                    name: String::from("dst"),
                },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::BitMatch {
                    name: String::from("s"),
                    value: true,
                },
                ParsingInstruction::Match {
                    size: 10,
                    value: 0b0011101000,
                },
                ParsingInstruction::Bit {
                    name: String::from("x"),
                },
                ParsingInstruction::Match { size: 5, value: 0 },
                ParsingInstruction::Register {
                    size: 5,
                    name: String::from("src"),
                },
                ParsingInstruction::ControlRegister {
                    size: 5,
                    name: String::from("crlo"),
                },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::BitMatch {
                    name: String::from("s"),
                    value: true,
                },
                ParsingInstruction::Match {
                    size: 10,
                    value: 0b0011111000,
                },
                ParsingInstruction::Bit {
                    name: String::from("x"),
                },
                ParsingInstruction::Unsigned {
                    size: 5,
                    name: String::from("crhi"),
                },
                ParsingInstruction::ControlRegister {
                    size: 5,
                    name: String::from("crlo"),
                },
                ParsingInstruction::Register {
                    size: 5,
                    name: String::from("dst"),
                },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::BitMatch {
                    name: String::from("s"),
                    value: true,
                },
                ParsingInstruction::Match {
                    size: 10,
                    value: 0b0011101000,
                },
                ParsingInstruction::Bit {
                    name: String::from("x"),
                },
                ParsingInstruction::Unsigned {
                    size: 5,
                    name: String::from("crhi"),
                },
                ParsingInstruction::Register {
                    size: 5,
                    name: String::from("src"),
                },
                ParsingInstruction::ControlRegister {
                    size: 5,
                    name: String::from("crlo"),
                },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
        ];
        for format in format_combinations {
            let Ok(parsed_variables) = parse(opcode, format.as_slice()) else {
                continue;
            };
            let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
            let control_register =
                ParsedVariable::try_get(&parsed_variables, "crlo")?.get_control_register()?;
            let (source, destination) = {
                if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "dst") {
                    let destination_register = variable.get_register()?;
                    (
                        RegisterFile::Control(control_register),
                        RegisterFile::GeneralPurpose(destination_register),
                    )
                } else if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "src") {
                    let source_register = variable.get_register()?;
                    (
                        RegisterFile::GeneralPurpose(source_register),
                        RegisterFile::Control(control_register),
                    )
                } else {
                    continue;
                }
            };
            let conditional_operation =
                ParsedVariable::try_get(&parsed_variables, "cond")?.get_conditional_operation()?;
            return Ok(Self {
                source,
                destination,
                unit: Unit::S,
                side: true,
                delayed: false,
                instruction_data: InstructionData {
                    opcode,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }
        Err(std::io::Error::other("Not MVC"))
    }
}

impl C64xInstruction for MoveRegisterInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        if let Ok(ret_val) = Self::new_mv(input.opcode) {
            return Ok(ret_val);
        } else if let Ok(ret_val) = Self::new_mvc(input.opcode) {
            return Ok(ret_val);
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Register instruction: No matches found."),
        ))
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        let mv_format = [
            ParsingInstruction::Bit {
                name: String::from("s"),
            },
            ParsingInstruction::Match {
                size: 2,
                value: 0b11,
            },
            ParsingInstruction::LSDUnit {
                name: String::from("unit"),
            },
            ParsingInstruction::Match { size: 1, value: 0 },
            ParsingInstruction::Bit {
                name: String::from("ms_bit"),
            },
            ParsingInstruction::Register {
                size: 3,
                name: String::from("src"),
            },
            ParsingInstruction::Unsigned {
                size: 2,
                name: String::from("ms"),
            },
            ParsingInstruction::Bit {
                name: String::from("x"),
            },
            ParsingInstruction::Register {
                size: 3,
                name: String::from("dst"),
            },
        ];

        let mvc_format = [
            ParsingInstruction::Bit {
                name: String::from("s"),
            },
            ParsingInstruction::Match {
                size: 6,
                value: 0b110111,
            },
            ParsingInstruction::Register {
                size: 3,
                name: String::from("src"),
            },
            ParsingInstruction::Match {
                size: 6,
                value: 0b110110,
            },
        ];

        if let Ok(parsed_variables) = parse(input.opcode, &mv_format) {
            let unit = ParsedVariable::try_get(&parsed_variables, "unit")?.get_unit()?;
            let side = ParsedVariable::try_get(&parsed_variables, "s")?.get_bool()?;
            let crosspath = ParsedVariable::try_get(&parsed_variables, "x")?.get_bool()?;
            let ms_bit = ParsedVariable::try_get(&parsed_variables, "ms_bit")?.get_bool()?;
            let ms = ParsedVariable::try_get(&parsed_variables, "ms")?.get_u8()?;
            let mut source_register =
                ParsedVariable::try_get(&parsed_variables, "src")?.get_register()?;
            let mut destination_register =
                ParsedVariable::try_get(&parsed_variables, "dst")?.get_register()?;
            if ms_bit {
                destination_register += (ms) << 3;
            } else {
                source_register += (ms) << 3;
            }
            if crosspath {
                source_register = !source_register;
            }
            let source = RegisterFile::GeneralPurpose(source_register);
            let destination = RegisterFile::GeneralPurpose(destination_register);
            return Ok(Self {
                source,
                destination,
                side,
                delayed: false,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    ..Default::default()
                },
            });
        } else if let Ok(parsed_variables) = parse(input.opcode, &mvc_format) {
            let side = ParsedVariable::try_get(&parsed_variables, "s")?.get_bool()?;
            let source_register =
                ParsedVariable::try_get(&parsed_variables, "src")?.get_register()?;
            let source = RegisterFile::GeneralPurpose(source_register);
            let destination = RegisterFile::Control(ControlRegister::ILC);
            return Ok(Self {
                source,
                destination,
                side,
                delayed: false,
                unit: Unit::S,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    ..Default::default()
                },
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Register instruction: No matches found."),
        ))
    }

    fn instruction_clean(&self) -> String {
        if self.destination.side() == None || self.source.side() == None {
            String::from("MVC")
        } else if self.delayed {
            String::from("MVD")
        } else {
            String::from("MV")
        }
    }

    fn instruction(&self) -> String {
        let mut value = format!(
            "{}.{}",
            self.instruction_clean(),
            self.unit.to_sided_string(self.side)
        );

        if self.destination.side() == Some(!self.side) || self.source.side() == Some(!self.side) {
            value += "X";
        }
        value
    }

    fn operands(&self) -> String {
        format!(
            "{}, {}",
            self.source.to_string(),
            self.destination.to_string()
        )
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
