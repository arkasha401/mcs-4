mod memory;
use memory::Memory;
mod cmp;
mod cpu;
use cpu::CPU;
use std::env;

pub fn main() {
    let filename: String = env::args().nth(1).unwrap();
    let mut c = cmp::assembler::Assembler::new();
    let mems = memory::Memory::new(c.compile(filename));
    let mut cpu = CPU::new(mems);
    cpu.run();
    cpu.display_info();
}
