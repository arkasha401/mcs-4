use std::collections::HashMap;

struct Stack {
    sp: u8,
    data: [u16; 3]
    
}
impl Stack {
    pub fn new() -> Stack{
        Stack {
            sp: 0,
            data: [0,0,0]
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp == 0 as u8 {
            panic!("ERROR: Trying to pop nothing");
        }
        self.sp -= 1;
        self.data[self.sp as usize]
    }

    pub fn push (&mut self, d:u16) {
        self.data[self.sp as usize] = d;
        self.sp += 1;
    }
    
    pub fn reset(&mut self){
        self.sp = 0;
        self.data = [0,0,0];


    }
}

pub struct CPU {
    registers: Registers,
    pc: u16,
    stack: Stack,
    accum: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 1, 
            stack: Stack::new(),
            accum: 0,
        }   
    }
    pub fn reset(&mut self) {
        self.registers.reset();
        self.pc = 0;
        self.stack.reset();
        self.accum = 0;
    
        
    }
}

struct Registers {
    r0: u8,
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
    r15: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0, 
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0, 
            r13: 0,
            r14: 0,
            r15: 0
        }
    }

        pub fn set_register(&mut self, val: u8, num_of_reg: u8){
            match num_of_reg {
                0 => self.r0 = val,
                1 => self.r1 = val,
                2 => self.r2 = val,
                3 => self.r3 = val,
                4 => self.r4 = val,
                5 => self.r5 = val,
                6 => self.r6 = val,
                7 => self.r7 = val,
                8 => self.r8 = val,
                9 => self.r9 = val,
                10 => self.r10 = val,
                11 => self.r11 = val,
                12 => self.r12 = val,
                13 => self.r13 = val,
                14 => self.r14 = val,
                15 => self.r15 = val,
                _ => std::process::exit(0)
            }
        }

        pub fn get_register(&self, num_of_reg: u8) -> u8 {
            match num_of_reg {
                0 => self.r0, 
                1 => self.r1,
                2 => self.r2,
                3 => self.r3,
                4 => self.r4,
                5 => self.r5,
                6 => self.r6,
                7 => self.r7,
                8 => self.r8,
                9 => self.r9,
                10 => self.r10,
                11 => self.r11,
                12 => self.r12,
                13 => self.r13, 
                14 => self.r14,
                15 => self.r15,
                _ => std::process::exit(0)
            }
        }  

        pub fn reset(&mut self) { 
            self.r0 = 0;
            self.r1 = 0;
            self.r2 = 0;
            self.r3 = 0;
            self.r4 = 0;
            self.r5 = 0;
            self.r6 = 0;
            self.r7 = 0;
            self.r8 = 0;
            self.r9 = 0;
            self.r10 = 0;
            self.r11 = 0;
            self.r12 = 0;
            self.r13 = 0;
            self.r14 = 0;
            self.r15 = 0;
        }
    }


