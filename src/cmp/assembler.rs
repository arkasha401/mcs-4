use crate::cmp::dictionary;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


pub struct Assembler {
    data: Vec<String>, 
    binary: Vec<u8>, 
} 

impl Assembler {
    pub fn new() -> Assembler {
        let name = env::args().nth(1).unwrap();
        let f = File::open(name).unwrap();
        let mut reader = BufReader::new(f);
        for (index, line) in reader.lines().enumerate() {
            for i in line.unwrap().split_whitespace() {
                println!("{}", i.trim())
            }

        } 

        Assembler {
            data: Vec::new(),
            binary: Vec::new()
        }
    }

    
}




