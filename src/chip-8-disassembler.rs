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

    //let code = &buffer[pc];
    let mut first = buffer[pc];
    let mut second = buffer[pc+1];
    print!("{:04x} {:02x} {:02x}", (pc + 0x200), first, second);
}
