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
    let firstNibble = buffer[pc] >> 4;
    print!("{:04x} {:02x} {:02x} ", (pc + 0x200), first, second);

    match firstNibble {
        0x0 =>
            match buffer[pc+1] {
                0x00 => print!("Empty Address"),
                0xe0 => print!("{:10}", "CLS"),
                0xee => print!("{:10}", "RET"),
                _ => print!("UNKNOWN 0 INSTRUCTION"),
            },
        0x01 => print!("{:10} ${:01x}{:01x}", "JUMP", buffer[pc] & 0xf, buffer[pc+1]),
        _=> print!("UNIMPLEMENTED INSTRUCTION"),
    }

}
