const MAX_ROM: usize = 4096;
const MAX_RAM: usize = 1024;

pub struct Memory {
    rom: [u8; MAX_ROM],
    ram: [u8; MAX_RAM]
}
impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom:[0;MAX_ROM],
            ram:[0;MAX_RAM]
        }
    }

    pub fn load_byte_rom(&mut self, adress: usize, value: u8){
        self.rom[adress] = value
    }

    pub fn get_byte_rom(&self, adress: usize) -> u8 {
        self.rom[adress]
    }

    pub fn load_byte_ram(&mut self, adress: usize, value:u8) {
        self.ram[adress] = value
    }

    pub fn get_byte_ram(&self, adress: usize) -> u8 {
        self.ram[adress]
     }

}