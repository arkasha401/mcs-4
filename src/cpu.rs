use crate::memory::Memory;
use core::panic;

const MAX_INDEX_REGISTERS: usize = 16;

#[derive(Debug)]
pub struct CPU {
    a_r: u8, //u4
    c_r: u8, //u1
    x2: u8,
    x3: u8,
    pc: u16,
    stack: [u16; 3],
    stack_p: u8, // u3
    index_registers: [u8; MAX_INDEX_REGISTERS],
    memory: Memory,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            a_r: 0,
            c_r: 0,
            x2: 0,
            x3: 0,
            pc: 0,
            stack: [0; 3],
            stack_p: 0, // u3
            index_registers: [0; MAX_INDEX_REGISTERS],
            memory: memory,
        }
    }
    pub fn run(&mut self) {
        loop {
            self.execute();
            //println!("{}, {}, {}, {:?}, {}, {:?}", self.a_r, self.c_r, self.pc, self.stack, self.stack_p, self.index_registers);
            if self.pc >= 2048 {
                break;
            }
        }
    }

    pub fn stack_push(&mut self, d: u16) -> () {
        self.stack[self.stack_p as usize] = d;
        self.stack_p += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        if self.stack_p == 0 as u8 {
            panic!("ERROR: Trying to pop nothing");
        }

        self.stack_p -= 1;
        self.stack[self.stack_p as usize]
    }

    pub fn execute(&mut self) {
        let instruction: (u8, u8) = self.fetch_opcode();
        self.decode(instruction);
        if self.pc >= 2048 {
            println!("It's done!");
        }
        self.pc += 1;
    }

    pub fn fetch_opcode(&mut self) -> (u8, u8) {
        let first_part: u8 = self.memory.rom.rom_get_word(self.pc as usize) >> 4;
        let second_part: u8 = self.memory.rom.rom_get_word(self.pc as usize) & 0b00001111;
        (first_part, second_part)
    }

    pub fn decode(&mut self, (opr, opa): (u8, u8)) {
        match opr {
            0 => (),
            1 => self.jcn_opr(opa),
            2 => match opa {
                0 => self.fim_opr(opa),
                1 => self.src_opr(opa),
                _ => panic!("ERROR: NO INSTRUCTION EXISTS"),
            },
            3 => match opa {
                0 => self.fin_opr(opa),
                1 => self.jin_opr(opa),
                _ => (),
            },
            4 => self.jun_opr(),
            5 => self.jms_opr(opa),
            6 => self.inc_opr(opa),
            7 => self.isz_opr(opa),
            8 => self.add_opr(opa),
            9 => self.sub_opr(opa),
            10 => self.ld_opr(opa),
            11 => self.xch_opr(opa),
            12 => self.bbl_opr(opa),
            13 => self.ldm_opr(opa),
            14 => match opa {
                0 => self.wrm_opr(),
                1 => self.wmp_opr(),
                2 => self.wrr_opr(),
                3 => (),
                4 => self.wr0_opr(),
                5 => self.wr1_opr(),
                6 => self.wr2_opr(),
                7 => self.wr3_opr(),
                8 => self.sbm_opr(),
                9 => self.rdm_opr(),
                10 => self.rdr_opr(),
                11 => self.adm_opr(),
                12 => self.rd_0(),
                13 => self.rd_1(),
                14 => self.rd_2(),
                15 => self.rd_3(),
                _ => panic!("ERROR! NO FOLLOW INSTRUCTION EXISTS"),
            },
            15 => match opa {
                0 => self.clb_opr(),
                1 => self.clc_opr(),
                2 => self.iac_opr(),
                3 => self.cmc_opr(),
                4 => self.cma_opr(),
                5 => self.ral_opr(),
                6 => self.rar_opr(),
                6 => self.rar_opr(),
                7 => self.tcc_opr(),
                8 => self.dac_opr(),
                9 => self.tcs_opr(),
                10 => self.stc_opr(),
                11 => self.daa_opr(),
                12 => self.kbp_opr(),
                // 13 => self.dcl_opr(),
                _ => panic!("ERROR! NO EXISTING INSTRUCTION"),
            },

            _ => panic!("NO FOLLOW INSTRUCTIONS"),
        }
    }
    pub fn ram_read_char(&self) -> u8 {
        let decided_reg: u8 = self.x2 & 0b0011;
        let char_pointer: u8 = self.x3;
        self.memory.ram.read_main_char(decided_reg, char_pointer)
    }

    pub fn ram_write_char(&mut self, value: u8) {
        let decided_reg: u8 = self.x2 & 0b0011;
        let char_pointer: u8 = self.x3;
        self.memory
            .ram
            .write_main_char(decided_reg, char_pointer, value)
    }

    pub fn ram_read_status(&mut self, status_pointer: u8) -> u8 {
        let decided_reg = self.x2 & 0b0011;
        self.memory
            .ram
            .read_status_char(decided_reg, status_pointer)
    }

    pub fn ram_write_status(&mut self, status_pointer: u8, value: u8) {
        let decided_reg = self.x2 & 0b0011;
        self.memory
            .ram
            .write_status_char(decided_reg, status_pointer, value)
    }

    pub fn ram_write_output(&mut self) {
        self.memory.ram.ram_write_output(self.a_r)
    }

    pub fn rom_read_port(&self) -> u8 {
        self.memory.rom.rom_read_port()
    }

    pub fn rom_write_port(&mut self) {
        self.memory.rom.rom_write_port(self.a_r)
    }

    // 1 word instructions

    pub fn ldm_opr(&mut self, opa: u8) {
        self.a_r = opa;
    }

    pub fn ld_opr(&mut self, opa: u8) {
        self.a_r = self.index_registers[opa as usize]
    }

    pub fn xch_opr(&mut self, opa: u8) {
        let temp: u8 = self.a_r;
        self.a_r = self.index_registers[opa as usize];
        self.index_registers[opa as usize] = temp
    }

    pub fn add_opr(&mut self, opa: u8) {
        if self.a_r + self.index_registers[opa as usize] + self.c_r > 15 {
            self.a_r += (self.index_registers[opa as usize] + self.c_r) & 0b1111;
            self.c_r = 1
        }
        self.a_r += (self.index_registers[opa as usize] + self.c_r) & 0b1111;
        self.c_r = 0
    }

    pub fn sub_opr(&mut self, opa: u8) {
        if self.c_r == 1 {
            self.a_r += (self.index_registers[opa as usize] + self.c_r) & 0b1111;
            self.c_r = 0;
        }
        self.a_r += (self.index_registers[opa as usize] + self.c_r) & 0b1111;
        self.c_r = 1
    }

    pub fn inc_opr(&mut self, opa: u8) {
        (self.index_registers[opa as usize] + 1) & 0b1111;
    }

    pub fn fin_opr(&mut self, opa: u8) {
        let (data1, data2) = self.fetch_opcode();
        self.index_registers[(opa & 0b1110) as usize] = data1;
        self.index_registers[(opa & 0b1111) as usize] = data2;
    }

    pub fn jin_opr(&mut self, opa: u8) {
        let ph: u16 = self.pc >> 8;
        let pm: u8 = self.index_registers[(opa as usize) & 0b1110];
        let pl: u8 = self.index_registers[(opa as usize) & 0b1111];
        self.pc = ph << 8 + (pm as u16) << 4 + pl as u16
    }

    pub fn src_opr(&mut self, opa: u8) {
        self.x2 = self.index_registers[opa as usize & 0b1110];
        self.x3 = self.index_registers[opa as usize & 0b1111];
    }

    pub fn bbl_opr(&mut self, opa: u8) {
        self.stack_pop();
        self.a_r = opa;
    }

    // Input/Output and RAM Instructions

    pub fn rdm_opr(&mut self) {
        self.a_r = self.ram_read_char();
    }

    pub fn rd_0(&mut self) {
        self.a_r = self.ram_read_status(0)
    }

    pub fn rd_1(&mut self) {
        self.a_r = self.ram_read_status(1)
    }

    pub fn rd_2(&mut self) {
        self.a_r = self.ram_read_status(2)
    }

    pub fn rd_3(&mut self) {
        self.a_r = self.ram_read_status(3)
    }

    pub fn wrm_opr(&mut self) {
        self.ram_write_char(self.a_r)
    }

    pub fn rdr_opr(&mut self) {
        self.a_r = self.rom_read_port()
    }

    pub fn wr0_opr(&mut self) {
        self.ram_write_status(0, self.a_r)
    }

    pub fn wr1_opr(&mut self) {
        self.ram_write_status(1, self.a_r)
    }

    pub fn wr2_opr(&mut self) {
        self.ram_write_status(2, self.a_r)
    }

    pub fn wr3_opr(&mut self) {
        self.ram_write_status(3, self.a_r)
    }

    pub fn wrr_opr(&mut self) {
        self.rom_write_port()
    }

    pub fn wmp_opr(&mut self) {
        self.ram_write_output()
    }

    pub fn adm_opr(&mut self) {
        self.a_r = self.ram_read_char() + self.a_r + self.c_r;
        self.a_r = self.a_r & 0b1111;
    }

    pub fn sbm_opr(&mut self) {
        self.a_r =
            (((self.a_r as i8) - (self.ram_read_char() as i8) - (self.c_r as i8)) & 0b1111) as u8;
    }

    // accumulator group instructions

    pub fn clb_opr(&mut self) {
        self.a_r = 0;
        self.c_r = 0;
    }

    pub fn clc_opr(&mut self) {
        self.c_r = 0;
    }

    pub fn cmc_opr(&mut self) {
        self.c_r ^= 1;
    }

    pub fn stc_opr(&mut self) {
        self.c_r = 1;
    }

    pub fn cma_opr(&mut self) {
        self.a_r = !self.a_r & 0b1111;
    }

    pub fn iac_opr(&mut self) {
        self.a_r += 1;
        if self.a_r <= 15 {
            self.c_r = 0
        }
    }

    pub fn dac_opr(&mut self) {
        self.a_r = self.a_r + 0b1111;
        if self.a_r > 15 {
            self.c_r = 1;
            self.a_r &= 0b1111
        }
        self.c_r = 0
    }

    pub fn ral_opr(&mut self) {
        self.a_r = (self.a_r << 4) + self.c_r;
        if self.a_r > 15 {
            self.c_r = 1;
            self.a_r &= 0b1111
        }
        self.c_r = 0;
        self.a_r = self.a_r & 0b1111;
    }

    pub fn rar_opr(&mut self) {
        self.c_r = self.a_r >> 3;
        self.a_r = self.a_r >> 1 + self.c_r << 3;
    }

    pub fn tcc_opr(&mut self) {
        self.a_r = self.c_r;
        self.c_r = 0;
    }

    pub fn daa_opr(&mut self) {
        if self.a_r > 9 || self.c_r == 1 {
            self.a_r += 6;
        }
        self.c_r = self.a_r >> 4;
        self.a_r &= 0b1111
    }

    pub fn tcs_opr(&mut self) {
        if self.c_r == 0 {
            self.a_r = 9;
            self.c_r = 0;
        }
        self.a_r = 10;
        self.c_r = 0;
    }

    pub fn kbp_opr(&mut self) {
        self.a_r = match self.a_r {
            0b0000 => 0b0000,
            0b0001 => 0b0000,
            0b0010 => 0b0000,
            0b0100 => 0b0000,
            0b1000 => 0b0000,
            _ => 0b1111,
        }
    }

    // pub fn dcl_opr(&mut self) {
    //     self.ram_bank = self.a_r & 0b111
    // }

    // 2 words instructions

    pub fn jun_opr(&mut self) {
        let (d1, d2) = self.fetch_opcode();
        self.pc = (d2 as u16) << 4 + (d1 as u16);
    }

    pub fn jms_opr(&mut self, opa: u8) {
        let (d1, d2) = self.fetch_opcode();
        self.pc = ((d2 as u16) << 4) + d1 as u16
    }

    pub fn jcn_opr(&mut self, opa: u8) {
        let (d1, d2) = self.fetch_opcode();
        let invert_condition = opa & 0b1000 == 0b1000;
        let condition: bool = ((opa & 0b0100 == 0b0100) && self.a_r == 0)
            || ((opa & 0b0010 == 0b0010) && self.c_r == 1)
            || ((opa & 0b0001 == 0b0001) && false);
        if (!condition && invert_condition) || (!invert_condition && condition) {
            let ph = self.pc >> 8;
            self.pc = (ph << 8) + ((d2 as u16) << 4) + (d1 as u16)
        }
    }

    pub fn isz_opr(&mut self, opa: u8) {
        let (d1, d2) = self.fetch_opcode();
        self.index_registers[opa as usize] = (self.index_registers[opa as usize] + 1) % 15;

        if self.index_registers[opa as usize] != 0 {
            let ph = self.pc >> 8;
            self.pc = (ph << 8) + ((d1 as u16) << 4) + (d2 as u16);
        }
    }

    pub fn fim_opr(&mut self, opa: u8) {
        let (d1, d2) = self.fetch_opcode();
        self.index_registers[opa as usize & 0b1110] = d1;
        self.index_registers[opa as usize & 0b1111] = d2;
    }

    pub fn display_info(&self) {
        println!(
            "a_r = {}, r0 = {}, r1 = {}",
            self.a_r, self.index_registers[0], self.index_registers[1]
        )
    }
}
