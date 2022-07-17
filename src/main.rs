mod memory;
mod cpu;

use std::env;

use cpu::CPU;


pub fn main() {
    let vector_of_instructions: Vec<u8> = vec![0b11011111, 0b11100000, 0b11100000];
    let mut mems = memory::Memory::new(vector_of_instructions);
    let mut cpu = CPU::new(mems);
    cpu.run();
    println!("{:?}", mems.rom.data) 
}
