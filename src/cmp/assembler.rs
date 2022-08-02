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
        let reader = BufReader::new(f);
        let mut temp_vec: Vec<String> = Vec::new();
        for (_index,line) in reader.lines().enumerate() {
            let _line = line.unwrap();
            let line = _line.trim();
            println!("{}", line);
            if !line.is_empty(){
                if line.trim().contains(";")  {
                    if *line.as_bytes().last().unwrap() == (';' as u8) { 
                        let temp_temp: Vec<_> = line.split_whitespace().collect(); 
                        for element in temp_temp {
                            if element.contains(';') {
                                if (element.as_bytes())[0] == ';' as u8 {
                                    ()
                                } else if *element.as_bytes().last().unwrap() == (';' as u8){
                                    temp_vec.push(element.replace(';', ""));
                            }
                            } else if !element.contains(';') {
                                temp_vec.push(element.to_string())
                            }
                            
                        println!("{:?}", temp_vec);
                        }
                    } else {
                        panic!("ERROR: SYNTAX ERROR")
                    }  
                } else if !line.contains(";"){
                    panic!("SYNTAX ERROR: ';' MISSED")
                }
            } else {
                continue;
            }
        }

        
        Assembler  {
            data: temp_vec,
            binary: Vec::new(),
            Dictionary: dictionary::Instructions::new() 
        }
    }


    pub fn assembly(&mut self) {
        let mut ptr: u16 = 0;
        for ptr in 0..self.data.len() {
            

        }
    }
}

impl Default for Assembler <'static> {
    fn default() -> Self {
        Self::new()
    }
}


    




