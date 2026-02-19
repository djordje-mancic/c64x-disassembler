# c64x-disassembler
Disassembler for Texas Instruments TMS320C64x and TMS320C64x+ Digital Signal Processors.

The disassembler mostly tries to follow the same syntax used in Texas Instruments' Code Composer Studio, with some exceptions where changes have been made for improved readability.

## Usage
You can use the disassembler either as a library or as a standalone program.
### Standalone Program
The command format is `c64x-disassembler [OPTIONS] <FILE>`

**Example:** ``c64x-disassembler CODE.bin``

All of the available options can be printed with ``c64x-disassembler --help``

### Library
You can add the disassembler as a dependency by running ``cargo add c64x-disassembler``

## Building
To build the disassembler, you will need to have Cargo (the Rust package manager) installed. 

Afterwards, you should be able to build by running ``cargo build`` in the main directory.