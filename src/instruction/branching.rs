use std::{
    cmp::min,
    io::{Error, ErrorKind},
};

use crate::instruction::{
    C64xInstruction, ConditionalOperation, InstructionData,
    parser::{ParsedVariable, ParsingInstruction, parse},
    register::{ControlRegister, Register},
};

pub enum BranchUsing {
    Displacement(i32),
    Register(Register),
    Pointer(ControlRegister),
}

pub struct BranchInstruction {
    instruction_data: InstructionData,
    pub branch_using: BranchUsing,
    pub side: bool,
    pce1_address: Option<u32>,
    pub nop_count: u8,
}

impl BranchInstruction {
    pub fn set_pce1_address(&mut self, address: u32) {
        self.pce1_address = Some(address);
    }
}

impl C64xInstruction for BranchInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats = [
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::Bit {
                    name: String::from("s"),
                },
                ParsingInstruction::Match {
                    size: 5,
                    value: 0b100,
                },
                ParsingInstruction::Signed {
                    size: 21,
                    name: String::from("cst"),
                },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::Bit {
                    name: String::from("s"),
                },
                ParsingInstruction::Match {
                    size: 10,
                    value: 0xD8,
                },
                ParsingInstruction::Bit {
                    name: String::from("x"),
                },
                ParsingInstruction::Match { size: 5, value: 0 },
                ParsingInstruction::RegisterCrosspath {
                    size: 5,
                    name: String::from("src"),
                },
                ParsingInstruction::Match { size: 5, value: 0 },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::Match {
                    size: 17,
                    value: 0x71,
                },
                ParsingInstruction::Unsigned {
                    size: 3,
                    name: String::from("op"),
                },
                ParsingInstruction::Match { size: 7, value: 0 },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::Bit {
                    name: String::from("s"),
                },
                ParsingInstruction::Match {
                    size: 11,
                    value: 0x48,
                },
                ParsingInstruction::Unsigned {
                    size: 3,
                    name: String::from("nop"),
                },
                ParsingInstruction::Signed {
                    size: 12,
                    name: String::from("cst"),
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
                    value: 0xD8,
                },
                ParsingInstruction::Bit {
                    name: String::from("x"),
                },
                ParsingInstruction::Unsigned {
                    size: 3,
                    name: String::from("nop"),
                },
                ParsingInstruction::Match { size: 2, value: 0 },
                ParsingInstruction::RegisterCrosspath {
                    size: 5,
                    name: String::from("src"),
                },
                ParsingInstruction::Match { size: 5, value: 1 },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("cond"),
                },
            ],
        ];
        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format.as_slice()) else {
                continue;
            };
            let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;
            let side = ParsedVariable::try_get(&parsed_variables, "s")?.get_bool()?;
            let conditional_operation =
                ParsedVariable::try_get(&parsed_variables, "cond")?.get_conditional_operation()?;
            let nop_count = {
                if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "nop") {
                    variable.get_u8()?
                } else {
                    0
                }
            };
            let branch_using = {
                if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "cst") {
                    BranchUsing::Displacement(
                        variable.get_i32()? << {
                            if nop_count > 0 && input.fphead.is_some() {
                                1
                            } else {
                                2
                            }
                        },
                    )
                } else if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "src") {
                    BranchUsing::Register(variable.get_register()?)
                } else if let Ok(variable) = ParsedVariable::try_get(&parsed_variables, "op") {
                    let opcode = variable.get_u8()?;
                    match opcode {
                        0b110 => BranchUsing::Pointer(ControlRegister::IRP),
                        0b111 => BranchUsing::Pointer(ControlRegister::NRP),
                        _ => continue,
                    }
                } else {
                    continue;
                }
            };
            return Ok(Self {
                side,
                branch_using,
                pce1_address: None,
                nop_count,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: false,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Not a branch instruction",
        ))
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        let Some(fphead) = &input.fphead else {
            return Err(Error::new(ErrorKind::InvalidInput, "No fphead"));
        };
        if !fphead.decode_compact_branches {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Decoding compact branch instructions set to false (BR = 0)",
            ));
        }

        let formats = [
            (
                "sbs7",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 5,
                        value: 0x5,
                    },
                    ParsingInstruction::Signed {
                        size: 7,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("nop"),
                    },
                ],
            ),
            (
                "sbu8",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 5,
                        value: 0x5,
                    },
                    ParsingInstruction::Unsigned {
                        size: 8,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b11,
                    },
                ],
            ),
            (
                "scs10",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 5,
                        value: 0xD,
                    },
                    ParsingInstruction::Signed {
                        size: 10,
                        name: String::from("cst"),
                    },
                ],
            ),
            (
                "sbs7c",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 3,
                        value: 0x5,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("z"),
                    },
                    ParsingInstruction::Match { size: 1, value: 1 },
                    ParsingInstruction::Signed {
                        size: 7,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("nop"),
                    },
                ],
            ),
            (
                "sbu8c",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 3,
                        value: 0x5,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("z"),
                    },
                    ParsingInstruction::Match { size: 1, value: 1 },
                    ParsingInstruction::Unsigned {
                        size: 8,
                        name: String::from("cst"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b11,
                    },
                ],
            ),
            (
                "sx1b",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 6,
                        value: 0x37,
                    },
                    ParsingInstruction::Unsigned {
                        size: 4,
                        name: String::from("src"),
                    },
                    ParsingInstruction::Match { size: 2, value: 0 },
                    ParsingInstruction::Unsigned {
                        size: 3,
                        name: String::from("nop"),
                    },
                ],
            ),
        ];

        for (name, format) in formats {
            let Ok(parsed_variables) = parse(input.opcode, format.as_slice()) else {
                continue;
            };
            let side = ParsedVariable::try_get(&parsed_variables, "s")?.get_bool()?;
            let branch_using = {
                if name == "sx1b" {
                    let num = ParsedVariable::try_get(&parsed_variables, "src")?.get_u8()?;
                    BranchUsing::Register(Register::B(num))
                } else {
                    BranchUsing::Displacement(if name == "sbu8" || name == "sbu8c" {
                        (ParsedVariable::try_get(&parsed_variables, "cst")?.get_u8()? as i32) << 1
                    } else {
                        ParsedVariable::try_get(&parsed_variables, "cst")?.get_i32()? << {
                            if name == "scs10" { 2 } else { 1 }
                        }
                    })
                }
            };
            let nop_count = {
                if name == "sbs7" || name == "sbs7c" || name == "sx1b" {
                    min(
                        ParsedVariable::try_get(&parsed_variables, "nop")?.get_u8()?,
                        5,
                    )
                } else {
                    5
                }
            };
            let conditional_operation = {
                if name == "scs10" {
                    Some(ConditionalOperation::ReservedLow)
                } else if name == "sbs7c" || name == "sbu8c" {
                    let z = ParsedVariable::try_get(&parsed_variables, "z")?.get_bool()?;
                    if z {
                        Some(ConditionalOperation::Zero(Register::from(0, side)))
                    } else {
                        Some(ConditionalOperation::NonZero(Register::from(0, side)))
                    }
                } else {
                    None
                }
            };
            return Ok(Self {
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    conditional_operation,
                    ..Default::default()
                },
                branch_using,
                side,
                pce1_address: None,
                nop_count,
            });
        }

        Err(Error::new(
            ErrorKind::InvalidInput,
            "Not a branch instruction",
        ))
    }

    fn instruction_clean(&self) -> String {
        if let Some(co) = self.conditional_operation()
            && co == ConditionalOperation::ReservedLow
        {
            String::from("CALLP")
        } else {
            if self.nop_count > 0 {
                String::from("BNOP")
            } else {
                String::from("B")
            }
        }
    }

    fn instruction(&self) -> String {
        let unit_num = if self.side { 2 } else { 1 };
        let mut instruction = format!("{}.S{unit_num}", self.instruction_clean());
        if let BranchUsing::Register(register) = self.branch_using
            && register.side() != self.side
        {
            instruction += "X";
        }
        instruction
    }

    fn operands(&self) -> String {
        let operands = match self.branch_using {
            BranchUsing::Displacement(displacement) => {
                let displacement_abs = displacement.unsigned_abs();
                let Some(address) = self.pce1_address else {
                    return String::from("ERROR - PCE1 ADDRESS EMPTY");
                };
                let branch_address = {
                    if displacement.is_positive() {
                        address + displacement_abs
                    } else {
                        address - displacement_abs
                    }
                };
                format!(
                    "0x{:08X} (PCE1{}0x{displacement_abs:08X})",
                    branch_address,
                    if displacement.is_positive() { "+" } else { "-" }
                )
            }
            BranchUsing::Register(register) => register.to_string(),
            BranchUsing::Pointer(register) => register.to_string(),
        };

        if let Some(co) = self.conditional_operation()
            && co == ConditionalOperation::ReservedLow
        {
            format!("{operands}, {}", Register::from(3, self.side).to_string())
        } else if self.nop_count > 0 {
            format!("{operands}, {}", self.nop_count)
        } else {
            operands
        }
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
