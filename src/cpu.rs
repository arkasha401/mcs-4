use crate::memory;

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

    pub fn execute(&mut self, mem: &mut memory::Memory) {

    } 
}


