use crate::cmp::dictionary;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Assembler<'a> {
    data: Vec<String>,
    binary: Vec<u8>,
    Dictionary: dictionary::Instructions<'a>,
}

impl Assembler<'static> {
    pub fn new() -> Assembler<'static> {
        let name = env::args().nth(1).unwrap();
        let f = File::open(name).unwrap();
        let reader = BufReader::new(f);
        let mut temp_vec: Vec<String> = Vec::new();
        for (_index, line) in reader.lines().enumerate() {
            let _line = line.unwrap();
            let mut line = _line.trim().to_string();
            if !line.is_empty() {
                if line.chars().last().unwrap() == ';' {
                    println!("{}", line);
                    line.pop();
                    println!("{}", line)
                } else {
                    println!("wtf")
                }
            } else {
                continue;
            }
        }

        Assembler {
            data: temp_vec,
            binary: Vec::new(),
            Dictionary: dictionary::Instructions::new(),
        }
    }

    pub fn assembly(&mut self) {
        let mut ptr: u16 = 0;
        for ptr in 0..self.data.len() {}
    }
}

impl Default for Assembler<'static> {
    fn default() -> Self {
        Self::new()
    }
}
