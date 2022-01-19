use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::ops::{Index, IndexMut};
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
        let instr = self.memory_read(self.regs[Register::PC]);
        // load one instruction from ram[pc]
        // pc++
        // determine instruction
        // perform instruction
    }

    // TODO:
    fn memory_read(&self, pc: u16) -> u16 {
        return pc;
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

pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

impl Index<Register> for [u16] {
    type Output = u16;

    fn index(&self, reg: Register) -> &Self::Output {
        &self[reg as usize]
    }
}

impl IndexMut<Register> for [u16] {
    fn index_mut(&mut self, reg: Register) -> &mut Self::Output {
        &mut self[reg as usize]
    }
}

pub enum Flag {
    POS = 1 << 0,
    ZERO = 1 << 1,
    NEG = 1 << 2,
}

pub enum Trap {
    GETC = 0x20,  //
    OUT = 0x21,   //
    PUTS = 0x22,  //
    IN = 0x23,    //
    PUTSP = 0x24, //
    HALT = 0x25,  //
}
