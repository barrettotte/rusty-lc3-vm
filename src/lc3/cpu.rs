use std::ops::{Index, IndexMut};

const RAM_SIZE: usize = 65536;

pub struct Cpu {
    ram: [u16; RAM_SIZE],
    regs: [u16; 10],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            regs: [0; 10],
        }
    }

    pub fn ram(&self) -> &[u16] {
        &self.ram
    }

    pub fn regs(&self) -> &[u16] {
        &self.regs
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
