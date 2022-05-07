mod memory;
mod cpu;
mod instructions;
use std::env;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rom = memory::Memory::new();
    rom.load_byte_rom(0, 212);
    let mut execution = cpu::CPU::new();
    execution.execute(&mut rom);
    let instruction_set =  instructions::Instructions::new();
}
