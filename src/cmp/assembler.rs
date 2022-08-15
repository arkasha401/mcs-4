use crate::cmp::dictionary;
use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Assembler<'a> {
    asm_code: Vec<String>,
    binary: Vec<u8>,
    Dictionary: dictionary::Instructions<'a>,
}

impl Assembler<'static> {
    pub fn new() -> Assembler<'static> {
        Assembler {
            asm_code: Vec::new(),
            binary: Vec::new(),
            Dictionary: dictionary::Instructions::new(),
        }
    }

    pub fn read_file(filename: String) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut data: Vec<String> = Vec::new();
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            data.push(line);
        }
        data
    }

    pub fn compile(&mut self, file_name: String) {
        self.asm_code = Assembler::read_file(file_name);

        for (c, asm_str) in self.asm_code.iter().enumerate() {
            println!("{:>0wid$X}\t{}", c, asm_str, wid = 2);
        }
    }
}
