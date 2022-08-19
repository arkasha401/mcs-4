use std::collections::HashMap;

pub struct Instructions<'a> {
    pub opcodes: HashMap<&'a str, u8>,
    pub opcodes_lenght: [Vec<&'a str>; 3],
}

impl Instructions<'_> {
    pub fn new() -> Instructions<'static> {
        let mut opcodes = HashMap::<&str, u8>::new();
        opcodes.insert("JCN", 0x1);
        opcodes.insert("FIM", 0x2);
        opcodes.insert("SRC", 0x2);
        opcodes.insert("FIN", 0x3);
        opcodes.insert("JIN", 0x3);
        opcodes.insert("JUN", 0x40);
        opcodes.insert("JMS", 0x5);
        opcodes.insert("INC", 0x6);
        opcodes.insert("ISZ", 0x70);
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
        opcodes.insert("RD0", 0xEC);
        opcodes.insert("RD1", 0xED);
        opcodes.insert("CLC", 0xF1);
        opcodes.insert("CLB", 0xF0);
        opcodes.insert("IAC", 0xF2);
        opcodes.insert("CMC", 0xF3);
        opcodes.insert("CMA", 0xF4);
        opcodes.insert("RAL", 0xF5);
        opcodes.insert("RAR", 0xF6);
        opcodes.insert("TCC", 0xF7);
        opcodes.insert("DAC", 0xF8);
        opcodes.insert("TCS", 0xF9);
        opcodes.insert("STC", 0xFA);
        opcodes.insert("DAA", 0xFB);
        opcodes.insert("KBP", 0xFC);

        let one_word = vec![
            "RDM", "RD0", "RD1", "RD2", "RD3", "RDR", "WRM", "WR0", "WR1", "WR2", "WR3", "WRR",
            "WMP", "ADM", "SBM", "CLB", "CLC", "CMC", "STC", "CMA", "IAC", "DAC", "RAL", "RAR",
            "TCC", "DAA", "TCS", "KBP",
        ];
        let two_words = vec![
            "JUN", "LDM", "LD", "XCH", "ADD", "SUB", "INC", "BBL", "JIN", "SRC", "FIN",
        ];
        let three_words = vec!["JMS", "JCN", "ISZ", "FIM"];

        let opcodes_lenght: [Vec<&str>; 3] = [one_word, two_words, three_words];
        Instructions {
            opcodes,
            opcodes_lenght,
        }
    }
}
// let with_one_operand: Vec<String> = ;
// let without_operands: Vec<String> = vec[1] ;
// let with_two_opr: Vec<String> = ;

impl Default for Instructions<'static> {
    fn default() -> Self {
        Self::new()
    }
}
