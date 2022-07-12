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
    pub fn new() -> ROM {
        ROM {
            data: [0; ROM_SIZE],
            io: 0 
        }
    }


    pub fn rom_read_word(&self, adress: usize) -> u8 {
        if adress > 255 {
            panic!("ERROR: Adress is out of range!") 
        }
        self.data[adress]
    }
}

impl Default for ROM {
    fn default() -> Self {
        Self::new()
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