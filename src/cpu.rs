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

pub struct Instructions {
    pub instruction_set: HashMap<String, u8>,
    instruction_type: [Vec<String>; 3],
    instruction_lenght: [Vec<String>; 2],
}

impl Default for Instructions {
    fn default() -> Self {
        Instructions::new()
    }
}

impl Instructions {
    pub fn new() -> Instructions {
        Instructions {
            instruction_set: Instructions::opcodes(),
            instruction_type: Instructions::instruction_type(),
            instruction_lenght: Instructions::instruction_lenght(),
            }
        }

    pub fn opcodes() -> HashMap<String, u8>{
        let mut instruction_set: HashMap<String, u8> = HashMap::new();
        let mut counter: u8 = 0;
        for reg1 in 0..16 {
            for reg2 in 0..16 {
                let opc = reg1 + reg2; 
                if reg1 == 0x0 {
                    instruction_set.insert("NOP".to_string(), opc);
                } else if reg1 == 0x1 {
                    instruction_set.insert("JCN".to_string(), opc);
                } else if reg1 == 0x2 {
                    match reg2 { 
                        0 => instruction_set.insert("FIM".to_string(), opc),
                        1 => instruction_set.insert("SRC".to_string(), opc),
                        _ => break
                    };
                } else if reg1 == 3 { 
                    match reg2 {
                        0x0 => instruction_set.insert("FIN".to_string(), opc),
                        0x1 => instruction_set.insert("JIM".to_string(), opc),
                        _ => break
                    };
                } else if reg1 == 4 {
                    instruction_set.insert("JUN".to_string(), opc);
                } else if reg1 == 5 { 
                    instruction_set.insert("JMS".to_string(), opc);
                } else if reg1 == 6 {
                    instruction_set.insert("INC".to_string(), opc);
                } else if reg1 == 7 {
                    instruction_set.insert("ISZ".to_string(), opc);
                } else if reg1 == 8 { 
                    instruction_set.insert("ADD".to_string(), opc);
                } else if reg1 == 9 {
                    instruction_set.insert("SUB".to_string(), opc); 
                } else if reg1 == 10 { 
                    instruction_set.insert("LD".to_string(), opc);
                } else if reg1 == 11 { 
                    instruction_set.insert("XCH".to_string(), opc);
                } else if reg1 == 12 { 
                    instruction_set.insert("BBL".to_string(), opc);
                } else if reg1 == 13 { 
                    instruction_set.insert("LDM".to_string(), opc);
                } else if reg1 == 14 {
                    match reg2 {
                        0x0 => instruction_set.insert("WRM".to_string(), opc),
                        0x1 => instruction_set.insert("WMP".to_string(), opc),
                        0x2 => instruction_set.insert("WRR".to_string(), opc),
                        0x3 => instruction_set.insert("WPM".to_string(), opc),
                        0x4 => instruction_set.insert("WR0".to_string(), opc),
                        0x5 => instruction_set.insert("WR1".to_string(), opc),
                        0x6 => instruction_set.insert("WR2".to_string(), opc),
                        0x7 => instruction_set.insert("WR3".to_string(), opc),
                        0x8 => instruction_set.insert("SBM".to_string(), opc),
                        0x9 => instruction_set.insert("RDM".to_string(), opc),
                        0xA => instruction_set.insert("RDR".to_string(), opc),
                        0xB => instruction_set.insert("ADM".to_string(), opc),
                        0xC => instruction_set.insert("RD0".to_string(), opc),
                        0xD => instruction_set.insert("RD1".to_string(), opc), 
                        0xE => instruction_set.insert("RD2".to_string(), opc),
                        0xF => instruction_set.insert("RD3".to_string(), opc),
                        _ => break
                    };
                } else if reg1 == 15 {
                    match reg2 { 
                        0 => instruction_set.insert("CLB".to_string(), opc),
                        1 => instruction_set.insert("CLC".to_string(), opc),
                        2 => instruction_set.insert("IAC".to_string(), opc),
                        3 => instruction_set.insert("CMC".to_string(), opc),
                        5 => instruction_set.insert("RAL".to_string(), opc),
                        6 => instruction_set.insert("RAR".to_string(), opc),
                        7 => instruction_set.insert("TCC".to_string(), opc),
                        8 => instruction_set.insert("DAC".to_string(), opc),
                        9 => instruction_set.insert("TCS".to_string(), opc),
                        10 => instruction_set.insert("STC".to_string(), opc),
                        11 => instruction_set.insert("DAA".to_string(), opc),
                        12 => instruction_set.insert("KBP".to_string(), opc),
                        13 => instruction_set.insert("DCL".to_string(), opc),
                        _ => continue
                    };
                }
            }
        }
        instruction_set
    }
    pub fn instruction_lenght() -> [Vec<String>; 2] {
        let one_byte_instr: Vec<String> = vec![
            "NOP".to_string(),
            "SRC".to_string(),
            "FIN".to_string(),
            "JIN".to_string(),
            "INC".to_string(),
            "ADD".to_string(),
            "SUB".to_string(),
            "LD".to_string(),
            "XCH".to_string(),
            "BBL".to_string(),
            "LDM".to_string(),
            "WRM".to_string(),
            "WMP".to_string(),
            "WRR".to_string(),
            "WR0".to_string(),
            "WR1".to_string(),
            "WR2".to_string(),
            "WR3".to_string(),
            "SBM".to_string(),
            "RDM".to_string(),
            "RDR".to_string(),
            "ADM".to_string(),
            "RD0".to_string(),
            "RD1".to_string(),
            "RD2".to_string(),
            "RD3".to_string(),
            "CLB".to_string(),
            "CLC".to_string(),
            "IAC".to_string(),
            "CMC".to_string(),
            "CMA".to_string(),
            "RAL".to_string(),
            "RAR".to_string(),
            "TCC".to_string(),
            "DAC".to_string(),
            "TCS".to_string(),
            "STC".to_string(),
            "DAA".to_string(),
            "KBP".to_string(),
            "DCL".to_string() 
        ];

        let two_byte_instr: Vec<String> = vec![
            "JUN".to_string(),
            "JMS".to_string(), 
            "JCN".to_string(),
            "ISZ".to_string(),
            "FIM".to_string()
        ];
        
        [one_byte_instr, two_byte_instr]
    }

    pub fn instruction_type() -> [Vec<String>; 3]{
        let machine_instructions: Vec<String> = vec![
            "NOP".to_string(),
            "LDM".to_string(),
            "LD".to_string(),
            "XCH".to_string(),
            "ADD".to_string(),
            "SUB".to_string(),
            "INC".to_string(),
            "BBL".to_string(),
            "JIN".to_string(),
            "SRC".to_string(),
            "FIN".to_string(),
            "JUN".to_string(),
            "JMS".to_string(),
            "JCN".to_string(),
            "ISZ".to_string(),
            "FIM".to_string()
        ];

        let io_instructions: Vec<String> = vec![
            "RDM".to_string(),
            "RD0".to_string(),
            "RD1".to_string(),
            "RD2".to_string(),
            "RD3".to_string(),
            "RDR".to_string(),
            "WRM".to_string(),
            "WR0".to_string(),
            "WR1".to_string(),
            "WR2".to_string(),
            "WR3".to_string(),
            "WRR".to_string(),
            "WMP".to_string(),
            "ADM".to_string(),
            "SBM".to_string()
        ];

        let accum_instructions: Vec<String> = vec![
            "CLB".to_string(),
            "CLC".to_string(),
            "CMC".to_string(),
            "STC".to_string(),
            "CMA".to_string(),
            "IAC".to_string(),
            "DAC".to_string(),
            "RAL".to_string(),
            "RAR".to_string(),
            "TCC".to_string(),
            "DAA".to_string(),
            "TCS".to_string(),
            "KBP".to_string(),
            "DCL".to_string(),

        ];
        
        [machine_instructions, io_instructions, accum_instructions]
    }

    pub fn run_instructions(self, cycles: u32, instruction: String) {
        if self.instruction_set.contains_key(&instruction) { 

        }
    }
}
