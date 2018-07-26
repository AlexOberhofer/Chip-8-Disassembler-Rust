use std::env;
use std::io;
use std::u8;
use std::vec::Vec;
use std::io::Read;
use std::fs::File;

fn main() -> io::Result<()>{

    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).expect("Unable to read file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut pc = 0;
    let max = buffer.len();

    while pc < max {
        //disassemble
        disassemble(&buffer, pc);
        pc += 2;
        print!("\n");
    }

    Ok(())
}

fn disassemble(buffer: &[u8], pc: usize) {

    let first = buffer[pc];
    let second = buffer[pc+1];
    let first_nibble = buffer[pc] >> 4;
    let last_nibble = buffer[pc + 1] & 0xf;
    print!("{:04x} {:02x} {:02x} ", (pc + 0x200), first, second);

    match first_nibble {
        0x0 =>
            match buffer[pc+1] {
                0x00 => print!("Empty Address"),
                0xe0 => print!("{:10}", "CLS"),
                0xee => print!("{:10}", "RET"),
                _ => print!("UNKNOWN 0 INSTRUCTION"),
            },
        0x01 => print!("{:10} ${:01x}{:02x}", "JUMP", buffer[pc] & 0xf, buffer[pc+1]),
        0x02 => print!("{:10} V{:01X}, ${:02x}", "CALL", buffer[pc] & 0xf, buffer[pc+1]),
        0x03 => print!("{:10} V{:01X}, ${:02x}", "SKIP.EQI", buffer[pc] & 0xf, buffer[pc+1]),
        0x04 => print!("{:10} V{:01X}, ${:02x}", "SKIP.NEI", buffer[pc] & 0xf, buffer[pc+1]),
        0x05 => print!("{:10} V{:01X}, V{:02X}", "SKIP.EQ", buffer[pc] & 0xf, buffer[pc+1] >> 4),
        0x06 => print!("{:10} V{:01X}, ${:02x}", "MVI", buffer[pc] & 0xf, buffer[pc+1]),
        0x07 => print!("{:10} V{:02X}, ${:02x}", "ADDI", buffer[pc] & 0xf, buffer[pc+1]),
        0x08 =>
            match last_nibble {
                0x0 => print!("{:10} V{:01X} V{:01X}", "LD.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x1 => print!("{:10} V{:01X} V{:01X}", "OR.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x2 => print!("{:10} V{:01X} V{:01X}", "AND.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x3 => print!("{:10} V{:01X} V{:01X}", "XOR.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x4 => print!("{:10} V{:01X} V{:01X}", "ADD.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x5 => print!("{:10} V{:01X} V{:01X}", "SUB.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x6 => print!("{:10} V{:01X} V{:01X}", "SHR.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0x7 => print!("{:10} V{:01X} V{:01X}", "SUBN.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                0xe => print!("{:10} V{:01X} V{:01X}", "SHL.", buffer[pc] & 0xf, buffer[pc+1] >> 4),
                _=> print!("Unknown 8 Instruction"),
            },
        0x09 => print!("{:10} V{:01X} V{:01X}", "SNE", buffer[pc] & 0xf, buffer[pc+1] >> 4),
        0x0a => print!("{:10} I, ${:01x}{:02x}", "LD ADR", buffer[pc] & 0xf, buffer[pc+1]),
        0x0b => print!("{:10} ${:01x}{:02x} + V(0)", "JUMP", buffer[pc] & 0xf, buffer[pc+1]),
        0x0c => print!("{:10} V{:01X} ${:02X}", "RND", buffer[pc] & 0xf, buffer[pc+1]),
        0x0d => print!("{:10} V{:01X} V{:01X} ${:01x}", "SPRITE", buffer[pc] & 0xf, buffer[pc + 1] >> 4, buffer[pc + 1] & 0xf),
        0x0e =>
            match buffer[pc + 1] {
                0x9e => print!("{:10} V{:01X}", "SKIPKEY E", buffer[pc] & 0xf),
                0xa1 => print!("{:10} V{:01X}", "SKIPKEY NE", buffer[pc] & 0xf),
                _=> print!("Unknown E Instruction"),
            },
        0x0f =>
            match buffer[pc + 1]{
                0x07 => print!("{:10} V{:01X}, DT", "MOV", buffer[pc] & 0xf),
                0x0a => print!("{:10} V{:01X}", "WAIT KEY", buffer[pc] & 0xf),
                0x15 => print!("{:10} DEL, V{:01X}", "MOV", buffer[pc] & 0xf),
                0x18 => print!("{:10} SND, V{:01X}", "MOV", buffer[pc] & 0xf),
                0x1e => print!("{:10} I, V{:01X}", "ADD", buffer[pc] & 0xf),
                0x29 => print!("{:10} I, V{:01X}", "LD", buffer[pc] & 0xf),
                0x33 => print!("{:10} (I), V{:01X}", "MOV BCD", buffer[pc] & 0xf),
                0x55 => print!("{:10} (I), V0-V{:01X}", "MEM", buffer[pc] & 0xf),
                0x65 => print!("{:10} V0-V{:01X}, (I)", "MEM", buffer[pc] & 0xf),
                _=> print!("Unknown F Instruction"),
            },


        _=> print!("UNIMPLEMENTED INSTRUCTION"),
    }

}
