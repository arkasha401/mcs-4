const RAM_NUM_OF_REGISTERS: usize = 4;
const RAM_MAIN_MEMORY_CHARS: usize = 16;
const RAM_STATUS_CHARS: usize = 4;
const NUMBER_OF_REGISTERS: usize = 4;
const ROM_SIZE: usize = 256;
const MAX_RAM: usize = 512;




pub struct Register {
    main_memory: [u8; RAM_MAIN_MEMORY_CHARS],
    status_memory: [u8; RAM_STATUS_CHARS]
}

impl Register {

}


pub struct RAM {
    register: Register,
    output: u8
}

impl RAM {

}

pub struct ROM {
    data: [u8; ROM_SIZE],
    io: u8
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
    
    pub fn rom_read_word(&self, adress: usize) -> u8 {
        if adress > 255 {
            panic!("ERROR: Adress is out of range!") 
        }
        self.data[adress]
    }

    pub fn rom_write_port(&mut self, value: u8) -> () {
        self.io = value
    }

    pub fn rom_read_port(&self) -> u8 {
        self.io
    }


}




impl Default for ROM {
    fn default() -> Self {
        Self::new(vec![0;255])
    }
}


pub struct Memory {
    pub ram: RAM,
    pub rom: ()
}



// impl Memory {
//     pub fn new() -> Memory {
//         Memory {
            
//         }
//     }



// }