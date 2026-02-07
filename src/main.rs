use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
    process::exit,
};

use clap::Parser;

use crate::disasm::{PACKET_SIZE, read_packet};

mod disasm;
mod instruction;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    file: String,
    /// Memory offset to apply to addresses
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
        if let Err(e) = reader.read_exact(&mut buf) {
            if e.kind() != ErrorKind::UnexpectedEof {
                eprintln!("Error reading from file: {e}");
                exit(-1);
            }
            break;
        }
        let Ok(instructions) = read_packet(buf) else {
            address += PACKET_SIZE as u32;
            continue;
        };
        for instruction in instructions {
            println!(
                "0x{address:08X}: {:<12} {:<12} {}",
                format!("{:X}", instruction.opcode()),
                instruction.instruction(),
                instruction.operands()
            );
            if instruction.is_compact() {
                address += 2;
            } else {
                address += 4;
            }
        }
    }
}
