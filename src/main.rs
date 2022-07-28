mod memory;
use memory::Memory;
mod cpu;
mod cmp;

use cpu::CPU;


pub fn main() {
    let mems = memory::Memory::new(vec![0b0000,0b0000]);
    let mut cpu = CPU::new(mems);
    cpu.run();
    
}
