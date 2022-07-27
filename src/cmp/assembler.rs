use crate::cmp::dictionary;


pub struct Assembler {
    data: Vec<String>, 
    binary: Vec<u8>, 
} 

impl Assembler {
    pub fn new(data: Vec<String>) -> Assembler {
        Assembler {
            data,
            binary: Vec::new()
        }
    }
    pub fn assembly(&mut self) {
        
    }
}


