mod memory;
pub fn main() {
    let mut rom = memory::Memory::new();
    rom.load_byte_rom(0, 9);
    println!("{}",rom.get_byte_rom(0))
}