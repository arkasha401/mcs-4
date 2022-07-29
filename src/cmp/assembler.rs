use crate::cmp::dictionary; 
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;



pub struct Assembler <'a>{
    data: Vec<String>, 
    binary: Vec<u8>, 
    Dictionary: dictionary::Instructions <'a> 
}

impl Assembler <'static> {
    pub fn new() -> Assembler <'static> {
        let name = env::args().nth(1).unwrap();
        let f = File::open(name).unwrap();
        let mut temp_vec: Vec<String> = Vec::new();
        let reader = BufReader::new(f);
        for (_index,line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains(";")  {
                if *line.as_bytes().last().unwrap() == (';' as u8) {
                    temp_vec.push(line.replace(';', "").trim().to_string());
                    println!("{:?}", temp_vec);
                }    
            } else if !line.contains(";"){
                panic!("SYNTAX ERROR: ';' MISSED")
            }


        }
        Assembler  {
            data: Vec::new(),
            binary: Vec::new(),
            Dictionary: dictionary::Instructions::new() 
        }
    }
}

impl Default for Assembler <'static> {
    fn default() -> Self {
        Self::new()
    }
}


    




