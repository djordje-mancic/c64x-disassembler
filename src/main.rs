use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;

use crate::disasm::{PACKET_SIZE, read_packet};

mod disasm;
mod instruction;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.file).expect("Couldn't open file");
    let mut reader = BufReader::new(file);
    let mut address = 0u32;
    loop {
        let mut buf = [0u8; PACKET_SIZE];
        reader.read_exact(&mut buf).expect("Couldn't read exact");
        let Ok(packets) = read_packet(buf) else {
            address += PACKET_SIZE as u32;
            continue;
        };
        for packet in packets {
            println!(
                "0x{address:08X}: {:<12} {:<12} {}",
                format!("{:X}", packet.opcode()),
                packet.instruction(),
                packet.operands()
            );
            address += packet.amount_bytes();
        }
    }
}
