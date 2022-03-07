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
    rom.set(0, 0xA0);
    cpu.reset();
    let run_instr = Instructions::new();
    println!("{:?}", run_instr.instruction_set)
}
