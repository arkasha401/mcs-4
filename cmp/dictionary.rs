use std::collections::HashMap;


pub struct Instructions {
    opcodes: HashMap<&str, u8>
}

impl Instructions {
    pub fn new() -> Instructions {
        let mut opcodes = HashMap::<&str, u8>::new();
        opcodes.insert("JCN", 0x1);
        opcodes.insert("FIM", 0x2);
        opcodes.insert("SRC", 0x2);
        opcodes.insert("FIN", 0x3);
        opcodes.insert("JIN", 0x3);
        opcodes.insert("JUN", 0x4);
        opcodes.insert("JMS", 0x5);
        opcodes.insert("INC", 0x6);
        opcodes.insert("ISZ", 0x7);
        opcodes.insert("ADD", 0x8);
        opcodes.insert("SUB", 0x9);
        opcodes.insert("LD", 0xA);
        opcodes.insert("XCH", 0xB);
        opcodes.insert("BBL", 0xC);
        opcodes.insert("LDM", 0xD);
        opcodes.insert("WRM", 0xE);
        opcodes.insert("WMP", 0xE);
        opcodes.insert("WRR", 0xE);
        opcodes.insert("WPM", 0xE);
        opcodes.insert("WR0", 0xE);
        opcodes.insert("WR1", 0xE);
        opcodes.insert("WR2", 0xE);
        opcodes.insert("WR3", 0xE);
        opcodes.insert("SBM", 0xE);
        opcodes.insert("RDM", 0xE);
        opcodes.insert("RDR", 0xE);
        opcodes.insert("ADM", 0xE);
        opcodes.insert("RD0", 0xE);
        opcodes.insert("RD1", 0xE);
        opcodes.insert("RD2", 0xE);
        opcodes.insert("RD3", 0xE);
        opcodes.insert("CLB", 0xF);
        opcodes.insert("CLC", 0xF);
        opcodes.insert("IAC", 0xF);
        opcodes.insert("CMC", 0xF);
        opcodes.insert("CMA", 0xF);
        opcodes.insert("RAL", 0xF);
        opcodes.insert("RAR", 0xF);
        opcodes.insert("TCC", 0xF);
        opcodes.insert("DAC", 0xF);
        opcodes.insert("TCS", 0xF);
        opcodes.insert("STC", 0xF);
        opcodes.insert("DAA", 0xF);
        opcodes.insert("KBP", 0xF);
        opcodes.insert("DCL", 0xF);
        Instructions
    } 
}