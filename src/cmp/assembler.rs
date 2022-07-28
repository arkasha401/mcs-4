use crate::cmp::dictionary;
use std::fs::File;
use std::io::{BufRead, BufReader};


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

    pub fn read_file(filepath: &str){
        let r_file = File::open(filepath).unwrap();  
        let reader = BufReader::new(r_file);

        for (index, line) in reader.lines().enumerate(){
            

        } 
    }  
    pub fn assembly(&mut self) {
        
    }
}


