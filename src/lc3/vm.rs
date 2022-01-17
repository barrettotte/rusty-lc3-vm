use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Index, IndexMut};

const PC_START: u16 = 0x3000;
const RAM_SIZE: usize = u16::MAX as usize;

#[derive(Debug)]
pub struct VirtualMachine {
    ram: [u16; RAM_SIZE],
    regs: [u16; 10],
    is_halted: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            regs: [0; 10],
            is_halted: false,
        }
    }

    pub fn run(&mut self) {
        while !self.is_halted {
            // TODO: cycle
            for n in 1..10 {
                println!("{}", n);
            }
            self.is_halted = true;
        }
        println!("{}", self);
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

pub enum OpCode {
    BR,   // 0000 | conditional branch
    ADD,  // 0001 | addition
    LD,   // 0010 | load
    ST,   // 0011 | store
    JSR,  // 0100 | jump to subroutine
    AND,  // 0101 | bitwise logical AND
    LDR,  // 0110 | load register (base + offset)
    STR,  // 0111 | store register (base + offset)
    RTI,  // 1000 | return from interrupt
    NOT,  // 1001 | bitwise complement
    LDI,  // 1010 | load indirect
    STI,  // 1011 | store indirect
    JMP,  // 1100 | jump / return from subroutine
    RES,  // 1101 | unused; illegal opcode
    LEA,  // 1110 | load effective address
    TRAP, // 1111 | system call
}
