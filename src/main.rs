use std::env;

use rumdump::rumdis;
use rumdump::rumload;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    println!("{} instructions", instructions.len());
    let mut i = 0;
    for instruction in instructions {
        println!("{}: [{:x}] {}", i, instruction, rumdis::disassemble(instruction));
        i+=1;
    }
}
