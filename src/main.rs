use std::env;

use rumdump::rumdis;
use rumdump::rumload;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    println!("{} instructions", instructions.len());
    println!("{}", rumdis::header());
    for (i, instruction) in instructions.iter().enumerate() {
        println!(
            "{:06}: [{:08x}] [{}] {}",
            i,
            instruction,
            rumdis::bin_string(*instruction),
            rumdis::disassemble(*instruction)
        );
    }
}
