const RAM_MAIN_MEMORY_CHARS: usize = 16;
const RAM_STATUS_CHARS: usize = 4;
const NUMBER_OF_REGISTERS: usize = 4;
const ROM_SIZE: usize = 2048;

#[derive(Copy, Clone, Debug)]
pub struct Register {
    main_memory: [u8; RAM_MAIN_MEMORY_CHARS],
    status_memory: [u8; RAM_STATUS_CHARS],
}
#[derive(Copy, Clone, Debug)]
pub struct RAM {
    registers: [Register; NUMBER_OF_REGISTERS],
    pub output: u8,
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            registers: [Register {
                main_memory: [0; RAM_MAIN_MEMORY_CHARS],
                status_memory: [0; RAM_STATUS_CHARS],
            }; NUMBER_OF_REGISTERS],
            output: 0,
        }
    }

    pub fn read_main_char(&self, register: u8, char_pointer: u8) -> u8 {
        self.registers[register as usize].main_memory[char_pointer as usize]
    }

    pub fn write_main_char(&mut self, register: u8, char_pointer: u8, value: u8) {
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

#[derive(Clone, Debug)]
pub struct ROM {
    pub binary_vec: [u8; ROM_SIZE],
    io: u8,
}

impl ROM {
    pub fn new(rom: Vec<u8>) -> ROM {
        let mut setup_rom: ROM = ROM {
            binary_vec: [0; ROM_SIZE],
            io: 0,
        };

        for x in 0..rom.len() {
            setup_rom.binary_vec[x] = rom[x];
        }

        setup_rom
    }

    pub fn rom_get_word(&self, adress: usize) -> u8 {
        if adress == ROM_SIZE {
            panic!("ERROR: Adress is out of range!")
        }
        self.binary_vec[adress]
    }

    pub fn rom_read_port(&self) -> u8 {
        self.io
    }

    pub fn rom_write_port(&mut self, value: u8) -> () {
        self.io = value
    }
}

#[derive(Clone, Debug)]
pub struct Memory {
    pub ram: RAM,
    pub rom: ROM,
}

impl Memory {
    pub fn new(instruction_list: Vec<u8>) -> Memory {
        Memory {
            ram: RAM::new(),
            rom: ROM::new(instruction_list),
        }
    }
}
