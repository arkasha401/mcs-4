use crate::memory;
use crate::instructions;
const MAX_INDEX_REGISTERS: usize = 16; 

pub struct CPU { 
    a_r: u8, //u4
    c_r: u8, //u1 
    pc: u16,
    stack: [u16;3],
    stack_p: u8, // u3
    index_registers: [u8; MAX_INDEX_REGISTERS]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a_r: 0,
            c_r: 0,
            pc: 0,
            stack: [0;3],
            stack_p: 0, // u3
            index_registers: [0;MAX_INDEX_REGISTERS]

        }
    }

    pub fn execute(&mut self, mem: &mut memory::Memory)  {
        let instruct: (u8, u8) = self.fetch_opcode(mem);
        let instr = self.decode(instruct);
    } 

    pub fn fetch_byte(&mut self, mem: &mut memory::Memory, adress: &usize) {
        let byte: u8 = mem.get_byte_ram(*adress);
    }


    pub fn fetch_opcode(&mut self, mem: &mut memory::Memory) -> (u8, u8) {
        let first_part: u8 = mem.get_byte_rom(self.pc as usize) << 4;
        let second_part: u8 = mem.get_byte_rom(self.pc as usize);
        (first_part, second_part)
        
    }

    pub fn decode(&mut self, (opr, opa): (u8,u8)) {
            match opr { 
                0 => (),
                10 => self.opr_ld(opa),
                11 => self.opr_xch(opa),
                13 => self.opr_ldm(opa),

                _ => ()
            }
    }

    pub fn opr_ldm(&mut self, opa:u8){ 
        self.a_r = opa;
    }

    pub fn opr_ld(&mut self, opa: u8) {
        self.a_r = self.index_registers[opa as usize]
    }

    pub fn opr_xch(&mut self, opa: u8) {
        let temp: u8 = self.a_r;
        self.a_r = self.index_registers[opa as usize];
        self.index_registers[opa as usize] = temp
    }
    

    pub fn opr_add(&mut self, opa: u8){
        if self.a_r + self.index_registers[opa as usize] + self.c_r > 15 {
            self.a_r = self.a_r & 0b1111;
            self.c_r = 1
        }
        self.a_r += self.index_registers[opa as usize] + self.c_r & 0b1111 ;
        self.c_r = 0

    }

    pub fn opr_sub(&mut self, opa: u8) {
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


    pub fn fin_opr(&mut self, opa: u8, mem: &mut memory::Memory) { 
        let (data1, data2) = self.fetch_opcode(mem);
        self.index_registers[opa as usize] = data1;
        self.index_registers[(opa + 1) as usize] = data2;
    } 

    pub fn jin_opr(&mut self, opa: u8) {
        let (d1, d2) = (self.index_registers[opa as usize], self.index_registers[opa as usize + 1]);
    } 
}
