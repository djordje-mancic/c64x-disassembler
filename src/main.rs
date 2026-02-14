use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, ErrorKind, Read, Write, stdin, stdout},
    path::PathBuf,
    process::exit,
};

use clap::Parser;

use crate::{
    disasm::{
        COMPACT_INSTRUCTION_SIZE, INSTRUCTION_SIZE, PACKET_SIZE, read_instruction, read_packet,
    },
    instruction::{C64xInstruction, ConditionalOperation, InstructionInput},
};

mod disasm;
mod instruction;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    file: PathBuf,

    /// File path to output the disassembled code to.
    ///
    /// If unspecified, the code is outputted to
    /// standard output (stdout).
    #[arg(short, long, value_name = "OUTPUT_PATH")]
    output: Option<PathBuf>,

    /// Memory offset to apply to addresses.
    ///
    /// Note that this affects the packet fetching process.
    #[arg(short = 'O', long, default_value_t = 0)]
    offset: u32,
}

fn handle_output_file(args: &Args) -> Option<BufWriter<File>> {
    if let Some(path) = &args.output {
        if path.exists() {
            loop {
                print!("File at path {:?} already exists, overwrite? (Y/N): ", path);
                let _ = stdout().flush();
                let mut input_line = String::new();
                let _ = stdin().read_line(&mut input_line);
                match input_line.to_uppercase().as_str() {
                    "Y\n" => break,
                    "N\n" => exit(-1),
                    _ => (),
                }
            }
        }
        let file_result = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path);
        let Ok(file) = file_result else {
            let _ = file_result.inspect_err(|e| eprintln!("Couldn't create output file: {e}"));
            exit(-1);
        };
        Some(BufWriter::new(file))
    } else {
        None
    }
}

fn print_instruction(
    instruction: Box<dyn C64xInstruction>,
    address: &mut u32,
    output: &mut dyn Write,
) {
    let line = format!(
        "0x{address:08X}: {:<12}{:<4}{:<6} {:<12} {}\n",
        format!("{:X}", instruction.opcode()),
        {
            if instruction.is_parallel() {
                String::from("||")
            } else {
                String::new()
            }
        },
        {
            if let Some(operation) = instruction.conditional_operation()
                && operation != ConditionalOperation::ReservedLow
                && operation != ConditionalOperation::ReservedHigh
            {
                format!("[{:>3}]", operation.to_string())
            } else {
                String::new()
            }
        },
        instruction.instruction(),
        instruction.operands()
    );

    output
        .write_all(line.as_bytes())
        .expect("Unable to write to output");

    if instruction.is_compact() {
        *address += COMPACT_INSTRUCTION_SIZE as u32;
    } else {
        *address += INSTRUCTION_SIZE as u32;
    }
}

fn main() {
    let args = Args::parse();

    let file_result = File::open(&args.file);
    let Ok(file) = file_result else {
        let _ = file_result.inspect_err(|e| eprintln!("Couldn't open file: {e}"));
        exit(-1);
    };
    let mut reader = BufReader::new(file);
    let mut output_file = handle_output_file(&args);
    let output: &mut dyn Write = {
        if let Some(file) = &mut output_file {
            file
        } else {
            &mut stdout()
        }
    };

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

        if do_read_packet {
            if let Ok(packet_instructions) = read_packet(buf, address) {
                for instruction in packet_instructions {
                    print_instruction(instruction, &mut address, output);
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
            if let Ok(instruction) = read_instruction(InstructionInput::new(opcode)) {
                print_instruction(instruction, &mut address, output);
            } else {
                address += INSTRUCTION_SIZE as u32;
                continue;
            }
        }
    }
    output.flush().expect("Unable to flush");
}
