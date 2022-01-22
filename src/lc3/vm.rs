use super::opcodes::OpCode;
use super::registers::{Flag, Register};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

const PC_START: u16 = 0x3000;
const MEM_SIZE: usize = u16::MAX as usize;

#[derive(Debug)]
pub struct VirtualMachine {
    mem: [u16; MEM_SIZE],
    regs: [u16; 10],
    is_halted: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            mem: [0; MEM_SIZE],
            regs: [0; 10],
            is_halted: false,
        }
    }

    // load VM with LC3 program image
    pub fn load_img(&mut self, img_path: &Path) -> IoResult<()> {
        let mut f = File::open(img_path).expect("File not found");
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        for ins in buffer.iter() {
            println!("{}", ins);
        }

        Ok(())
    }

    // run VM until halted
    pub fn run(&mut self) {
        while !self.is_halted {
            self.cycle();
            self.is_halted = true;
        }
        println!("{}", self);
    }

    // perform one CPU cycle
    pub fn cycle(&mut self) {
        let instr = self.get_curr_ins();
        let op = OpCode::from_u16(instr >> 12);
        println!("{}", op);

        match op {
            OpCode::BR => op_br(),
            OpCode::ADD => op_add(&mut self.regs, instr),
            OpCode::LD => op_ld(),
            OpCode::ST => op_st(),
            OpCode::JSR => op_jsr(),
            OpCode::AND => op_and(),
            OpCode::LDR => op_ldr(),
            OpCode::STR => op_str(),
            OpCode::RTI => op_rti(),
            OpCode::NOT => op_not(),
            OpCode::LDI => op_ldi(),
            OpCode::STI => op_sti(),
            OpCode::JMP => op_jmp(),
            OpCode::RES => op_res(),
            OpCode::LEA => op_lea(),
            OpCode::TRAP => op_trap(),
        }
        // load one instruction from ram[pc]
        // pc++
        // determine instruction
        // perform instruction
    }

    // read cell at given address
    fn memory_read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    // get current instruction
    fn get_curr_ins(&self) -> u16 {
        self.memory_read(self.regs[Register::PC])
    }
}

impl Display for VirtualMachine {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{{\n    halt: {},\n    registers: {:02X?}\n}}",
            self.is_halted, self.regs
        )
    }
}

pub enum Trap {
    GETC = 0x20,  //
    OUT = 0x21,   //
    PUTS = 0x22,  //
    IN = 0x23,    //
    PUTSP = 0x24, //
    HALT = 0x25,  //
}

// update condition flags
pub fn update_flags(regs: &mut [u16], r: usize) {
    let test = regs[r];
    regs[Register::COND] = {
        if test == 0 {
            Flag::ZERO
        } else if (test >> 15) == 1 {
            Flag::NEG
        } else {
            Flag::POS
        }
    } as u16;
}

// sign extend value to n-bits
fn sign_extend(x: u16, bits: u8) -> u16 {
    if ((x >> (bits - 1)) & 1) == 1 {
        return x | (0xFFFF << bits);
    }
    return x;
}

fn op_br() {}

fn op_add(regs: &mut [u16], instr: u16) {
    let dr: usize = ((instr >> 9) & 0x7).into();
    let sr1: usize = ((instr >> 6) & 0x7).into();

    if (instr >> 5) & 0x1 == 1 {
        regs[dr] = regs[sr1] + sign_extend(instr & 0x1F, 5);
    } else {
        let sr2: usize = (instr & 0x7).into();
        regs[dr] = regs[sr1] + regs[sr2];
    }
    update_flags(regs, dr);
}

fn op_ld() {}

fn op_st() {}

fn op_jsr() {}

fn op_and() {}

fn op_ldr() {}

fn op_str() {}

fn op_rti() {}

fn op_not() {}

fn op_ldi() {}

fn op_sti() {}

fn op_jmp() {}

fn op_res() {}

fn op_lea() {}

fn op_trap() {}
