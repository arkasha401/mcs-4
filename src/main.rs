mod memory;
use memory::Memory;
mod cpu;
use cpu::CPU;
use cpu::Instructions;
fn main() {

    let mut cpu = CPU::new();
    let mut rom = Memory::new(4096);
    let mut ram = Memory::new(4096);
    rom.reset();
    rom.set(4095, 0xA0);
    rom.set(4096, 0xA1);
    cpu.reset();
    let run_instr = Instructions::run_instructions();
    
}
