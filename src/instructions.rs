use std::collections::HashMap;
pub struct Instructions {
    instruction_type: [Vec<String>; 3],
    instruction_lenght: [Vec<String>; 2],
    instruction_clocktime: [Vec<String>; 2]
}

impl Instructions {
    pub fn new() -> Instructions {
        Instructions {
            instruction_type: Instructions::instruction_type(),
            instruction_lenght: Instructions::instruction_lenght(),
            instruction_clocktime: Instructions::instruction_time(),
            }
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