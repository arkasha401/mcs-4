use crate::memory;
use crate::instructions;
const MAX_INDEX_REGISTERS:usize = 16; 

pub struct CPU { 
    a_r: u8, //u4
    c_r: bool,
    pc: u16,
    stack: [u16;3],
    r1: u8,
    r2: u8,
    r3: u8,
    r4: u8, 
    r5: u8,
    r6: u8,
    r7: u8,
    r8: u8,
    r9: u8,
    r10: u8,
    r11: u8,
    r12: u8,
    r13: u8,
    r14: u8,
    r15: u8,
    r16: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a_r: 0,
            c_r: false,
            pc: 0,
            stack: [0;3],
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0, 
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10:0,
            r11:0,
            r12:0,
            r13:0,
            r14:0,
            r15:0,
            r16:0,
        }
    }

    pub fn execute(&mut self, mem: &mut memory::Memory)  {
        let instruct: u8 = self.fetch_opcode(mem);
        let instr = self.decode(instruct);
    } 

    pub fn fetch_byte(&mut self, mem: &mut memory::Memory, adress: &usize) {
        let byte: u8 = mem.get_byte_ram(*adress);
    }


    pub fn fetch_opcode(&mut self, mem: &mut memory::Memory) -> u8 {
        let mut opcode: u8 = mem.rom[self.pc as usize];
        let opcode: u8 = mem.get_byte_rom(self.pc as usize);
        opcode
    }

    pub fn decode(&mut self, opcode: u8) {
        let upper_byte: u8 = (opcode >> 4) & 0b1111;
        let lower_byte: u8 = opcode & 0b1111;
            match upper_byte { 
                0 => (),
                10 => self.opr_ld(lower_byte),
                11 => self.opr_xch(lower_byte),
                13 => self.opr_ldm(lower_byte),
                _ => ()
            }
    }

    pub fn opr_ldm(&mut self, opa:u8){ 
        self.a_r = opa;
        println!("success")
    }

    pub fn opr_ld(&mut self, opa: u8) { 
            match opa { 
                1 => self.a_r = self.r1,
                2 => self.a_r = self.r2,
                3 => self.a_r = self.r3,
                4 => self.a_r = self.r4,
                5 => self.a_r = self.r5,
                6 => self.a_r = self.r6,
                7 => self.a_r = self.r7,
                8 => self.a_r = self.r8,
                9 => self.a_r = self.r9,
                10 => self.a_r = self.r10,
                11 => self.a_r = self.r11,
                12 => self.a_r = self.r12,
                13 => self.a_r = self.r13,
                14 => self.a_r = self.r14,
                15 => self.a_r = self.r15,
                16 => self.a_r = self.r16,
                _=> eprintln!("ERROR")
            }
    }
    pub fn opr_xch(&mut self, opa: u8) {
        let temp: u8 = self.a_r;
        match opa {
            1 => {self.a_r = self.r1; self.r1 = temp},
            2 => {self.a_r = self.r2; self.r2 = temp},
            3 => {self.a_r = self.r3; self.r3 = temp},
            4 => {self.a_r = self.r4; self.r4 = temp},
            5 => {self.a_r = self.r5; self.r5 = temp},
            6 => {self.a_r = self.r6; self.r6 = temp},
            7 => {self.a_r = self.r7; self.r7 = temp},
            8 => {self.a_r = self.r8; self.r8 = temp},
            9 => {self.a_r = self.r9; self.r9 = temp},
            10 => {self.a_r = self.r10; self.r10 = temp},
            11 => {self.a_r = self.r11; self.r11 = temp},
            12 => {self.a_r = self.r12; self.r12 = temp},
            13 => {self.a_r = self.r13; self.r13 = temp},
            14 => {self.a_r = self.r14; self.r14 = temp},
            15 => {self.a_r = self.r15; self.r15 = temp},
            16 => {self.a_r = self.r2; self.r2 = temp},
            _=> eprintln!("ERROR")
        } 
    }


}


