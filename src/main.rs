use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
    process::exit,
};

use clap::Parser;

use crate::{
    disasm::{
        COMPACT_INSTRUCTION_SIZE, INSTRUCTION_SIZE, PACKET_SIZE, read_instruction, read_packet,
    },
    instruction::C64xInstruction,
};

mod disasm;
mod instruction;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    file: String,
    /// Memory offset to apply to addresses.
    ///
    /// Note that this affects the packet fetching process.
    #[arg(short, long, default_value_t = 0)]
    offset: u32,
}

fn main() {
    let args = Args::parse();

    let file_result = File::open(&args.file);
    let Ok(file) = file_result else {
        let _ = file_result.inspect_err(|e| eprintln!("Couldn't open file: {e}"));
        exit(-1);
    };
    let mut reader = BufReader::new(file);

    let mut address = args.offset;
    loop {
        let mut buf = [0u8; PACKET_SIZE];
        let do_read_packet = if address % PACKET_SIZE as u32 == 0 {
            true
        } else {
            false
        };

        if let Err(e) = reader.read_exact(if do_read_packet {
            &mut buf
        } else {
            &mut buf[0..INSTRUCTION_SIZE]
        }) {
            if e.kind() != ErrorKind::UnexpectedEof {
                eprintln!("Error reading from file: {e}");
                exit(-1);
            }
            break;
        }

        let mut print_instruction = |instruction: Box<dyn C64xInstruction>| {
            println!(
                "0x{address:08X}: {:<12} {:<12} {}",
                format!("{:X}", instruction.opcode()),
                instruction.instruction(),
                instruction.operands()
            );
            if instruction.is_compact() {
                address += COMPACT_INSTRUCTION_SIZE as u32;
            } else {
                address += INSTRUCTION_SIZE as u32;
            }
        };

        if do_read_packet {
            if let Ok(packet_instructions) = read_packet(buf) {
                for instruction in packet_instructions {
                    print_instruction(instruction);
                }
            } else {
                address += PACKET_SIZE as u32;
                continue;
            }
        } else {
            let opcode_bytes = buf
                .first_chunk::<INSTRUCTION_SIZE>()
                .expect("Buf error")
                .clone();
            let opcode = u32::from_le_bytes(opcode_bytes);
            if let Ok(instruction) = read_instruction(opcode) {
                print_instruction(instruction);
            } else {
                address += INSTRUCTION_SIZE as u32;
                continue;
            }
        }
    }
}
