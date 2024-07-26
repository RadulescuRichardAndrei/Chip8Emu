use std::fs::File;
use std::io::Read;
use crate::cpu::Cpu;

mod stack;
mod cpu;
mod register;
mod memory;
mod decoded_instruction;
mod keypad;

fn main() {
    let path= "";
    let mut rom = File::open(path).expect("Unable to open file");
    let mut cpu= Cpu::new();
    let mut buffer= Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    cpu.initialize(&buffer);
    cpu.execute();

}
