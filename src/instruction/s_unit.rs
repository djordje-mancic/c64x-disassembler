use std::io::{Error, ErrorKind, Result};

use crate::instruction::{C64xInstruction, DestinationSide, Register};

pub struct SUnitInstruction {
    opcode: u32,
    name: String,
    pub parallel: bool,
    pub side: DestinationSide,
    pub csta: Option<u32>,
    pub cstb: Option<u32>,
    pub destination: Option<Register>,
    compact: bool
}

impl SUnitInstruction {
    pub fn new(opcode: u32) -> Result<Self> {
        let mut read_opcode = opcode;
        let mut instruction = SUnitInstruction::default();
        instruction.opcode = opcode;
        instruction.parallel = if read_opcode & 1 == 1 { true } else { false };
        read_opcode >>= 1;

        match (read_opcode >> 1) & 0b1111 {
            0b1010 => {
                move_constant(&mut instruction);
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Not an S Unit instruction",
                ));
            }
        }

        Ok(instruction)
    }

    pub fn new_compact(opcode: u16) -> Result<Self> {
        let mut instruction = SUnitInstruction::default();
        instruction.compact = true;
        instruction.opcode = opcode as u32;

        match (opcode >> 1) & 0b1111 {
            0b1001 => {
                move_constant_compact(&mut instruction);
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Not an S Unit compact instruction",
                ));
            }
        }

        Ok(instruction)
    }
}

impl C64xInstruction for SUnitInstruction {
    fn instruction(&self) -> String {
        let mut ret_str = self.name.clone() + ".S";
        if self.side == DestinationSide::A {
            ret_str += "1";
        } else {
            ret_str += "2";
        }
        ret_str
    }

    fn instruction_clean(&self) -> String {
        self.name.clone()
    }

    fn operands(&self) -> String {
        let mut ret_str = String::new();
        if let Some(csta) = self.csta {
            ret_str += format!("0x{csta:04X}, ").as_str()
        }
        if let Some(cstb) = self.cstb {
            ret_str += format!("0x{cstb:04X}, ").as_str()
        }
        if let Some(destination) = self.destination {
            ret_str += destination.to_string().as_str()
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

impl Default for SUnitInstruction {
    fn default() -> Self {
        Self {
            opcode: 0,
            name: String::from("UNKNOWN"),
            parallel: false,
            side: DestinationSide::A,
            csta: None,
            cstb: None,
            destination: None,
            compact: false
        }
    }
}

fn move_constant(instruction: &mut SUnitInstruction) {
    let mut read_opcode = instruction.opcode >> 1;
    if read_opcode & 1 == 1 {
        instruction.side = DestinationSide::B;
    }
    read_opcode >>= 5;
    instruction.name = {
        if read_opcode & 1 == 1 {
            String::from("MVKH")
        } else {
            String::from("MVK")
        }
    };
    read_opcode >>= 1;
    instruction.csta = Some(read_opcode & 0xFFFF);
    read_opcode >>= 16;
    instruction.destination = Some(Register::from_dest(
        (read_opcode & 0b11111) as u8,
        instruction.side,
    ));
}

fn move_constant_compact(instruction: &mut SUnitInstruction) {
    let mut read_opcode = instruction.opcode;
    instruction.name = String::from("MVK");

    if read_opcode & 1 == 1 {
        instruction.side = DestinationSide::B;
    }
    read_opcode >>= 5;

    let mut cst = 0u32;
    cst += (read_opcode & 0b11) << 5;
    read_opcode >>=2;

    instruction.destination = Some(Register::from_dest(
        (read_opcode & 0b111) as u8,
        instruction.side,
    ));

    read_opcode >>=3;
    cst += (read_opcode & 0b1) << 7;
    read_opcode >>=1;
    cst += (read_opcode & 0b11) << 3;
    read_opcode >>=2;
    cst += read_opcode & 0b111;
    instruction.csta = Some(cst);
}