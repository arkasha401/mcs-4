use std::fs::File;
use std::path::Path;
mod memory;
use memory::Memory;
mod cpu;
mod cmp;

use cpu::CPU;


pub fn main() {
    let file = 1;
    let vect: Vec<u8> = vec![0b00001111, 0b00001111];
    let mems = memory::Memory::new(vect);
    let mut cpu = CPU::new(mems);
    cpu.run();
    
}
