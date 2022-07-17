const RAM_NUM_OF_REGISTERS: usize = 4;
const RAM_MAIN_MEMORY_CHARS: usize = 16;
const RAM_STATUS_CHARS: usize = 4;
const NUMBER_OF_REGISTERS: usize = 4;
const ROM_SIZE: usize = 256;
const MAX_RAM: usize = 512;



#[derive(Copy, Clone)]
pub struct Register {
    main_memory: [u8; RAM_MAIN_MEMORY_CHARS],
    status_memory: [u8; RAM_STATUS_CHARS]
}

impl Register {
    pub fn new() -> Register {
        Register {
            main_memory: [0; RAM_MAIN_MEMORY_CHARS],
            status_memory: [0; RAM_STATUS_CHARS] 
        }
    }
}
#[derive(Copy, Clone)]

pub struct RAM {
    registers: [Register; NUMBER_OF_REGISTERS],
    pub output: u8
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            registers: [
                Register {
                    main_memory: [0; RAM_MAIN_MEMORY_CHARS],
                    status_memory: [0; RAM_STATUS_CHARS]
                };
            NUMBER_OF_REGISTERS],
            output: 0 
        }
    }

    pub fn read_main_char(&self,register: u8, char_pointer: u8,) -> u8 {
        self.registers[register as usize].main_memory[char_pointer as usize]
    }

    pub fn write_main_char(&mut self,register: u8, char_pointer: u8, value: u8) {
        self.registers[register as usize].main_memory[char_pointer as usize] = value
    }

    pub fn read_status_char(&mut self, register: u8, status_pointer: u8) -> u8 {
        self.registers[register as usize].status_memory[status_pointer as usize]
    }

    pub fn write_status_char(&mut self, register: u8, status_pointer: u8, value: u8) {
        self.registers[register as usize].status_memory[status_pointer as usize] = value  
    }

    pub fn ram_write_output(&mut self, value: u8) {
        self.output = value
    }

}

#[derive(Copy, Clone)]
pub struct ROM {
    pub data: [u8; ROM_SIZE],
    io: u8
}

impl Default for ROM {
    fn default() -> Self {
        Self::new(vec![0, ROM_SIZE as u8])
    }
}

impl ROM {
    pub fn new(rom: Vec<u8>) -> ROM {
        let mut setup_rom: ROM = ROM {
            data: [0; ROM_SIZE],
            io: 0 
        };

        (0..rom.len()).for_each(|x| {
            if x > 255 {
                panic!("ERROR! Index is out of range!")
            }
            setup_rom.data[x] = rom[x]
        });

        setup_rom
    }
    
    pub fn rom_get_word(&self, adress: usize) -> u8 {
        if adress > 255 {
            panic!("ERROR: Adress is out of range!") 
        }
        self.data[adress]
    }

    pub fn rom_read_port(&self) -> u8 {
        self.io
    }

    pub fn rom_write_port(&mut self, value: u8) -> () {
        self.io = value
    }
}


#[derive(Copy, Clone)]
pub struct Memory {
    pub ram: RAM,
    pub rom: ROM
}

impl Memory {
    pub fn new(instruction_list: Vec<u8>) -> Memory {
        Memory {
            ram: RAM::new(),
            rom: ROM::new(instruction_list)
        }
    }
}
