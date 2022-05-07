mod memory;
mod cpu;
mod intstructions;

pub fn main() {
    let mut rom = memory::Memory::new();
    rom.load_byte_rom(0, 208);
    let mut execution = cpu::CPU::new();
    execution.execute(&mut rom)
    
}
