use crate::cmp::dictionary;


pub struct Assembler {
    data: Vec<String>, 
    binary: Vec<u8>, 
} 

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            data: vec!["a".to_string(), "v".to_string()],
            binary: Vec::new()
        }
    }
}

