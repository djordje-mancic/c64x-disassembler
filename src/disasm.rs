use std::io::{Error, ErrorKind, Result};

use crate::instruction::{
    C64xInstruction, InvalidCompactInstruction, InvalidInstruction,
    fphead::CompactInstructionHeader, no_unit::NoUnitInstruction, s_unit::SUnitInstruction,
};

pub fn read_compact_instruction(opcode: u16) -> Result<Box<dyn C64xInstruction>> {
    if let Ok(instruction) = SUnitInstruction::new_compact(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = NoUnitInstruction::new_compact(opcode) {
        return Ok(Box::new(instruction));
    }

    Ok(Box::new(InvalidCompactInstruction::new(opcode)))
}

pub fn read_instruction(opcode: u32) -> Result<Box<dyn C64xInstruction>> {
    if let Ok(instruction) = SUnitInstruction::new(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = CompactInstructionHeader::new(opcode) {
        return Ok(Box::new(instruction));
    }

    if let Ok(instruction) = NoUnitInstruction::new(opcode) {
        return Ok(Box::new(instruction));
    }

    Ok(Box::new(InvalidInstruction::new(opcode)))
}

/// Size of a regular instruction in bytes
pub const INSTRUCTION_SIZE: usize = 4;
/// Size of a compact instruction in bytes
pub const COMPACT_INSTRUCTION_SIZE: usize = 2;
/// Size of an FP (Fetch Packet) in bytes
pub const PACKET_SIZE: usize = 8 * INSTRUCTION_SIZE;

pub fn read_packet(packet: [u8; PACKET_SIZE]) -> Result<Vec<Box<dyn C64xInstruction>>> {
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

    for i in 0..7 {
        let index_start = i * 4;
        if let Some(fphead) = fphead_option
            && fphead.layout[i]
        {
            vec.push(read_compact_instruction(u16::from_le_bytes([
                packet[index_start],
                packet[index_start + 1],
            ]))?);
            vec.push(read_compact_instruction(u16::from_le_bytes([
                packet[index_start + 2],
                packet[index_start + 3],
            ]))?);
        } else {
            let instruction = read_instruction(u32::from_le_bytes([
                packet[index_start],
                packet[index_start + 1],
                packet[index_start + 2],
                packet[index_start + 3],
            ]))?;
            if instruction.as_any().is::<CompactInstructionHeader>() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Compact instruction header found in unusual place",
                ));
            }
            vec.push(instruction);
        }
    }

    vec.push(last_instruction);
    Ok(vec)
}
