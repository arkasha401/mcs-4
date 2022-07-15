use crate::memory;
use memory::Memory; 

const MAX_INDEX_REGISTERS: usize = 16; 

pub struct CPU { 
    a_r: u8, //u4
    c_r: u8, //u1
    x2: u8,
    x3: u8,
    pc: u16,
    stack: [u16 ;3],
    stack_p: u8, // u3
    index_registers: [u8; MAX_INDEX_REGISTERS],
    memory: Memory
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            a_r: 0,
            c_r: 0,
            x2: 0, 
            x3: 0,
            pc: 0,
            stack: [0;3],
            stack_p: 0, // u3
            index_registers: [0;MAX_INDEX_REGISTERS],
            memory: memory
        }
    }

    pub fn run(&mut self, mem: &mut memory::ROM) {
        loop {
            self.execute(mem);
            if self.pc == 255 {
                break;
            }
        } 
    }
    
    pub fn push (&mut self, d:u16) -> () {
        self.stack[self.stack_p as usize] = d;
        self.stack_p += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.stack_p == 0 as u8 {
            panic!("ERROR: Trying to pop nothing");
        }
        
        self.stack_p -= 1;
        self.stack[self.stack_p as usize]
    }

    pub fn execute(&mut self, mem: &mut memory::ROM)  {
        let instruction: (u8, u8) = self.fetch_opcode(mem);
        self.decode(instruction);
        
        if self.pc == 255 {
            println!("It's done!");
        } else {
        self.pc += 1;     
        }
    } 
    // returning 4bit char
    pub fn fetch_char(&mut self, mem: &mut memory::Memory) -> u8 {
        let register_pointer = self.x2;
        let char_pointer: u8 = self.x3;
        if register_pointer > 3 {
            panic!("ERROR! 4002 REGISTER IS OUT OF RANGE!")
        }
        mem.ram.read_main_char(register_pointer, char_pointer)
    }

    pub fn fetch_opcode(&mut self, mem: &memory::ROM) -> (u8, u8) {
        let first_part: u8 = mem.rom_get_word(self.pc as usize) >> 4;
        let second_part: u8 = mem.rom_get_word(self.pc as usize) & 0b00001111;
        self.pc += 1 ;
        (first_part, second_part)
        
    }

    pub fn decode(&mut self, (opr, opa): (u8,u8)) {
            match opr { 
                0 => self.opr_nop(),
                2 => self.src_opr(opa),
                4 => self.fin_opr(opa, self.memory.rom),
                3 => self.jin_opr(opa),
                6 => self.inc_opr(opa),
                8 => self.add_opr(opa),
                9 => self.sub_opr(opa),
                10 => self.ld_opr(opa),
                11 => self.xch_opr(opa),
                12 => self.bbl_opr(opa),
                13 => self.ldm_opr(opa),

                _ => panic!("NO FOLLOW INSTRUCTIONS")
            }
    }

    pub fn opr_nop(&self) -> () {
        println!("NOP")
    }

    // 1 word instructions 

    pub fn ldm_opr(&mut self, opa:u8){ 
        self.a_r = opa;
    }

    pub fn ld_opr(&mut self, opa: u8) {
        self.a_r = self.index_registers[opa as usize]
    }

    pub fn xch_opr(&mut self, opa: u8) {
        let temp: u8 = self.a_r;
        self.a_r = self.index_registers[opa as usize];
        self.index_registers[opa as usize] = temp
    }

    pub fn add_opr(&mut self, opa: u8){
        if self.a_r + self.index_registers[opa as usize] + self.c_r > 15 {
            self.a_r = self.a_r & 0b1111;
            self.c_r = 1
        }
        self.a_r += self.index_registers[opa as usize] + self.c_r & 0b1111 ;
        self.c_r = 0
    }

    pub fn sub_opr(&mut self, opa: u8) {
        if self.c_r == 1 {
            self.a_r = self.a_r + self.index_registers[opa as usize] + self.c_r;
            self.c_r = 0 
        }
        self.a_r = self.a_r + self.index_registers[opa as usize] + self.c_r;
        self.c_r = 1;
    }

    pub fn inc_opr(&mut self, opa: u8) {
        self.index_registers[opa as usize] += 1;
        if self.index_registers[opa as usize] > 15 { 
            self.index_registers[opa as usize] = 0;
        }
    }

    pub fn fin_opr(&mut self, opa: u8, mem: memory::ROM) { 
        let (data1, data2) = self.fetch_opcode(&mem);
        self.index_registers[opa as usize] = data1;
        self.index_registers[(opa + 1) as usize] = data2;
    } 


    pub fn bbl_opr(&mut self, opa: u8) {
        self.pop();
        self.a_r = opa;
    }

    pub fn src_opr(&mut self, opa: u8) {
        self.x2 = self.index_registers[opa as usize & 0b1110];
        self.x3 = self.index_registers[opa as usize & 0b1111];

    }

    pub fn jin_opr(&mut self, opa: u8) {
        let ph: u16 = self.pc >> 8;
        let pm: u8 = self.index_registers[(opa as usize) & 0b1110];
        let pl: u8 = self.index_registers[(opa as usize) & 0b1111];
        self.pc = ph << 8 + (pm as u16) << 4 + pl as u16 

    }
    
    pub fn jun_opr(&mut self, opa: u8) {
        self.pop();
        self.a_r = opa
    }

        // 2 word instructions
}

