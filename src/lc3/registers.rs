use std::ops::{Index, IndexMut};

#[derive(Debug)]
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

// breakdown of COND register
pub enum Flag {
    POS = 1 << 0,
    ZERO = 1 << 1,
    NEG = 1 << 2,
}

// memory mapped register
pub enum MMRegister {
    KBSR = 0xFE00, // keyboard status register
    KBDR = 0xFE02, // keyboard data register
}
