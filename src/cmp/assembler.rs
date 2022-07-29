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
        let mut temp_vec: Vec<String> = Vec::new();
        let reader = BufReader::new(f);
        for (_index,mut line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains(";")  {
                if *line.clone().as_bytes().last().unwrap() == (';' as u8) {
                    println!(";");
                }
            } else {
            panic!("SYNTAX ERROR: ';' MISSED")
            }
        }
        Assembler {
            data: Vec::new(),
            binary: Vec::new()
        }
    }
}


    




