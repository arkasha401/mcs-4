use std::collections::HashMap;
pub struct Instructions {
    pub instruction_set: HashMap<String, u8>,
    instruction_type: [Vec<String>; 3],
    instruction_lenght: [Vec<String>; 2],
    instruction_clocktime: [Vec<String>; 2]
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
            instruction_clocktime: Instructions::instruction_time(),
            }
        }

    pub fn opcodes() -> HashMap<String, u8>{
        let mut instruction_set: HashMap<String, u8> = HashMap::new();
        for reg1 in (0..241).step_by(16){
            for reg2 in 0..16 {
                let opc = reg1 + reg2; 
                if reg1 == 0x00 {
                    instruction_set.insert("NOP".to_string(), opc);
                } else if reg1 == 0x10 {
                    instruction_set.insert("JCN".to_string(), opc);
                } else if reg1 == 0x20 {
                    match reg2 { 
                        0 => instruction_set.insert("FIM".to_string(), opc),
                        1 => instruction_set.insert("SRC".to_string(), opc),
                        _ => break
                    };
                } else if reg1 == 0x30 { 
                    match reg2 {
                        0x0 => instruction_set.insert("FIN".to_string(), opc),
                        0x1 => instruction_set.insert("JIM".to_string(), opc),
                        _ => break
                    };
                } else if reg1 == 0x40 {
                    instruction_set.insert("JUN".to_string(), opc);
                } else if reg1 == 0x50 { 
                    instruction_set.insert("JMS".to_string(), opc);
                } else if reg1 == 0x60 {
                    instruction_set.insert("INC".to_string(), opc);
                } else if reg1 == 0x70 {
                    instruction_set.insert("ISZ".to_string(), opc);
                } else if reg1 == 0x80 { 
                    instruction_set.insert("ADD".to_string(), opc);
                } else if reg1 == 0x90 {
                    instruction_set.insert("SUB".to_string(), opc); 
                } else if reg1 == 0xA0 { 
                    instruction_set.insert("LD".to_string(), opc);
                } else if reg1 == 0xB0 { 
                    instruction_set.insert("XCH".to_string(), opc);
                } else if reg1 == 0xC0 { 
                    instruction_set.insert("BBL".to_string(), opc);
                } else if reg1 == 0xD0 { 
                    instruction_set.insert("LDM".to_string(), opc);
                } else if reg1 == 0xE0 {
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
                } else if reg1 == 0xF0 {
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

    pub fn instruction_time() -> [Vec<String>; 2] {
        let two_clock: Vec<String> = vec! [
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
            "DCL".to_string(),
        ];
        let four_clock: Vec<String> = vec! [
            "JUN".to_string(),
            "JMS".to_string(), 
            "JCN".to_string(),
            "ISZ".to_string(),
            "FIM".to_string()
        ];
        [two_clock, four_clock]
    }
}