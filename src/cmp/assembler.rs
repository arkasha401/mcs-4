use crate::cmp::dictionary;
use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Assembler<'a> {
    binary: Vec<u8>,
    Dictionary: dictionary::Instructions<'a>,
}

impl Assembler<'static> {
    pub fn new() -> Assembler<'static> {
        let name = env::args().nth(1).unwrap();
        let f = File::open(name).unwrap();
        let reader = BufReader::new(f);
        let mut binary_vec: Vec<u8> = Vec::new();
        for (_index, line) in reader.lines().enumerate() {
            let _line = line.unwrap();
            let mut line = _line.trim().to_string();
            if !line.is_empty() {
                if line.chars().last().unwrap() == ';' {
                    line.pop();
                    //let temp_arr: Vec<&str> = line.split_whitespace().collect();
                    //let operand = temp_arr[1].as_bytes();
                    //println!("{:?}", operand);
                    match temp_arr[0] {
                        "LDM" => {
                            if temp_arr[1].as_bytes()[0] <= 15 {
                                binary_vec.push(0b11010000 + temp_arr[1].as_bytes()[0])
                            } else {
                                panic!()
                            }
                        }

                        _ => panic!("ERROR: INVALID SYNTAX"),
                    }
                } else {
                    panic!("ERROR: ';' MIGHT BE MISSED")
                }
            } else {
                continue;
            }
        }

        println!("{:?}", binary_vec);

        Assembler {
            binary: binary_vec,
            Dictionary: dictionary::Instructions::new(),
        }
    }
}
