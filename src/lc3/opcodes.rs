use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum OpCode {
    BR,   // 0x0 | conditional branch
    ADD,  // 0x1 | addition
    LD,   // 0x2 | load
    ST,   // 0x3 | store
    JSR,  // 0x4 | jump to subroutine
    AND,  // 0x5 | bitwise logical AND
    LDR,  // 0x6 | load register (base + offset)
    STR,  // 0x7 | store register (base + offset)
    RTI,  // 0x8 | return from interrupt
    NOT,  // 0x9 | bitwise complement
    LDI,  // 0xA | load indirect
    STI,  // 0xB | store indirect
    JMP,  // 0xC | jump / return from subroutine
    RES,  // 0xD | unused; illegal opcode
    LEA,  // 0xE | load effective address
    TRAP, // 0xF | system call
}

impl OpCode {
    pub fn from_u16(value: u16) -> OpCode {
        match value {
            0x0 => OpCode::BR,
            0x1 => OpCode::ADD,
            0x2 => OpCode::LD,
            0x3 => OpCode::ST,
            0x4 => OpCode::JSR,
            0x5 => OpCode::AND,
            0x6 => OpCode::LDR,
            0x7 => OpCode::STR,
            0x8 => OpCode::RTI,
            0x9 => OpCode::NOT,
            0xA => OpCode::LDI,
            0xB => OpCode::STI,
            0xC => OpCode::JMP,
            0xD => OpCode::RES,
            0xE => OpCode::LEA,
            0xF => OpCode::TRAP,
            _ => panic!("Unknown opcode: {}", value),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
