use crate::cmp::dictionary;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Assembler<'a> {
    asm_code: Vec<String>,
    program_counter: usize,
    binary: Vec<u8>,
    dictionary: dictionary::Instructions<'a>,
    labels_dict: HashMap<String, usize>,
}

impl Assembler<'static> {
    pub fn new() -> Assembler<'static> {
        Assembler {
            asm_code: Vec::new(),
            program_counter: 0,
            binary: Vec::new(),
            dictionary: dictionary::Instructions::new(),
            labels_dict: HashMap::new(),
        }
    }

    pub fn read_file(&mut self, filename: String) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut data: Vec<String> = Vec::new();
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            data.push(line);
        }
        data
    }

    pub fn precompile(&mut self, file_name: String) {
        self.asm_code = self.read_file(file_name);
        // for (c, asm_str) in self.asm_code.iter().enumerate() {
        //println!("{:>0wid$X}\t{}", c, asm_str, wid = 2);
        //}
        //
        //
        let tokens: Vec<String> = Vec::new();
        while self.program_counter < self.asm_code.len() {
            let mut current_line = self.asm_code[self.program_counter].clone();
            self.program_counter += 1;
            if current_line.is_empty() {
                continue;
            }
            current_line.replace("\t", " ");
            let tokens: Vec<String> = current_line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            if tokens[0].ends_with(':') {
                self.labels_dict
                    .insert(tokens[0].replace(":", "").clone(), self.program_counter);
            }
            println!("{:?}", self.labels_dict);
        }
    }

    pub fn compile(&mut self, filename: String) -> Vec<u8> {
        self.precompile(filename);
        self.program_counter = 0;
        while self.program_counter < self.asm_code.len() {
            let mut current_line = self.asm_code[self.program_counter].clone();
            self.program_counter += 1;
            if current_line.is_empty() {
                continue;
            }
            let tokens: Vec<String> = current_line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            println!("{:?}", tokens);
            if tokens[0] == "DONE:" {
                break;
            } else if tokens[0].ends_with(':') {
                continue;
            }
            if self.dictionary.opcodes_lenght[0].contains(&&tokens[0][..]) {
                if tokens.len() == 1 {
                    self.binary.push(self.dictionary.opcodes[&tokens[0][..]]);
                } else {
                    panic!("ERROR: TOO MANY OPERANDS");
                }
            } else if self.dictionary.opcodes_lenght[1].contains(&&tokens[0][..]) {
                if tokens.len() == 2 {
                    let mut temp = self.dictionary.opcodes[&tokens[0][..]] << 4;
                    if tokens[0] == "JUN".to_string()
                        && self.labels_dict.contains_key(&tokens[1][..])
                    {
                        temp += self.labels_dict[&tokens[1]] as u8;
                        self.binary.push(temp);
                        println!("{:?}", temp);
                    } else if tokens[1].parse::<u8>().unwrap() <= 15 {
                        temp += tokens[1].parse::<u8>().unwrap();
                        self.binary.push(temp);
                    }
                } else {
                    panic!("LINE: ERROR: TOO MANY OPERANDS");
                }
            } else if self.dictionary.opcodes_lenght[2].contains(&&tokens[0][..]) {
                if tokens.len() == 3 {
                    let mut temp = self.dictionary.opcodes[&tokens[0][..]];
                    if tokens[1].parse::<u8>().unwrap() <= 15 {
                        println!("a");
                        temp += tokens[1].parse::<u8>().unwrap();
                        self.binary.push(temp);
                        temp = 0;
                        if self.labels_dict.contains_key(&tokens[2][..]) {
                            let mut temp: u8 = 0;
                            temp += (self.labels_dict[&tokens[2][..]]) as u8;
                            println!("{}", temp);
                            self.binary.push(temp);
                        } else {
                            panic!("ERROR INVALID LABEL")
                        }
                    }
                }
            } else if tokens[0] == "" {
                panic!(
                    "LINE: {} ERROR: THIS INSTRUCTION DOESN'T EXIST.  {}",
                    self.program_counter, self.asm_code[self.program_counter]
                )
            }
        }
        for i in self.binary.clone() {
            println!("{:X}", i)
        }
        println!("{:?}", self.binary);
        self.binary.clone()
    }
}
