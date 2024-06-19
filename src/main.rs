use crate::cpu::Cpu;

mod stack;
mod cpu;
mod register;
mod memory;
mod decoded_instruction;
mod keypad;

fn main() {
    let mut cpu= Cpu::new();
    println!("Before {:?}", cpu.registers[20].get_register_value());
    let res = cpu.decode_instruction(0x3000);

    if let Err(e) = res{
        println!("Error: {}",e)
    }
    println!("After {:?}", cpu.registers[20].get_register_value());
}
