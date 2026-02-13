use std::io::{Error, ErrorKind, Result};

use crate::instruction::{
    C64xInstruction,
    branching::BranchInstruction,
    fphead::CompactInstructionHeader,
    invalid::InvalidInstruction,
    moving::{MoveConstantInstruction, MoveRegisterInstruction},
    nop::NOPInstruction,
};

pub fn read_compact_instruction(
    opcode: u16,
    fphead: &CompactInstructionHeader,
) -> Result<Box<dyn C64xInstruction>> {
    if let Ok(instruction) = MoveConstantInstruction::new_compact(opcode, fphead) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = MoveRegisterInstruction::new_compact(opcode, fphead) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = BranchInstruction::new_compact(opcode, fphead) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = NOPInstruction::new_compact(opcode, fphead) {
        return Ok(Box::new(instruction));
    }

    Ok(Box::new(InvalidInstruction::new_compact(opcode, fphead)?))
}

pub fn read_instruction(opcode: u32) -> Result<Box<dyn C64xInstruction>> {
    if let Ok(instruction) = MoveConstantInstruction::new(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = MoveRegisterInstruction::new(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = BranchInstruction::new(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = CompactInstructionHeader::new(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = NOPInstruction::new(opcode) {
        return Ok(Box::new(instruction));
    }

    Ok(Box::new(InvalidInstruction::new(opcode)?))
}

/// Size of a regular instruction in bytes
pub const INSTRUCTION_SIZE: usize = 4;
/// Size of a compact instruction in bytes
pub const COMPACT_INSTRUCTION_SIZE: usize = 2;
/// Size of an FP (Fetch Packet) in bytes
pub const PACKET_SIZE: usize = 8 * INSTRUCTION_SIZE;

pub fn read_packet(
    packet: [u8; PACKET_SIZE],
    address: u32,
) -> Result<Vec<Box<dyn C64xInstruction>>> {
    let mut vec: Vec<Box<dyn C64xInstruction>> = vec![];
    let last_instruction = read_instruction(u32::from_le_bytes([
        packet[PACKET_SIZE - 4],
        packet[PACKET_SIZE - 3],
        packet[PACKET_SIZE - 2],
        packet[PACKET_SIZE - 1],
    ]))?;
    let fphead_option = last_instruction
        .as_any()
        .downcast_ref::<CompactInstructionHeader>();

    let mut index = 0;
    while index < 7 * 4 {
        let instruction = {
            if let Some(fphead) = fphead_option
                && fphead.layout[index / 4]
            {
                read_compact_instruction(
                    u16::from_le_bytes([packet[index], packet[index + 1]]),
                    fphead,
                )?
            } else {
                read_instruction(u32::from_le_bytes([
                    packet[index],
                    packet[index + 1],
                    packet[index + 2],
                    packet[index + 3],
                ]))?
            }
        };

        if instruction.as_any().is::<CompactInstructionHeader>() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Compact instruction header found in unusual place",
            ));
        }

        if instruction.is_compact() {
            index += 2;
        } else {
            index += 4;
        }
        vec.push(instruction);
    }

    vec.push(last_instruction);

    for instruction in &mut vec {
        if let Some(branch_instruction) =
            instruction.as_any_mut().downcast_mut::<BranchInstruction>()
        {
            branch_instruction.set_pce1_address(address);
        }
    }

    Ok(vec)
}
