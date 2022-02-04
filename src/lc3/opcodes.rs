use super::registers::{Flag, MMRegister, Register};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{stdin, Read};

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

// conditional branch
pub fn op_br(instr: u16, regs: &mut [u16]) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond = (instr >> 9) & 0x7F; // NZP

    if (cond & regs[Register::COND]) > 0 {
        regs[Register::PC] += pc_offset;
    }
}

// addition
pub fn op_add(instr: u16, regs: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let sr1 = ((instr >> 6) & 0x7) as usize;
    let is_imm = (instr >> 5) & 0x1;

    if is_imm == 1 {
        regs[dr] = regs[sr1] + sign_extend(instr & 0x1F, 5);
    } else {
        let sr2: usize = (instr & 0x7).into();
        regs[dr] = regs[sr1] + regs[sr2];
    }
    update_flags(regs, dr);
}

// load
pub fn op_ld(instr: u16, regs: &mut [u16], mem: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let offset = sign_extend(instr & 0x1FF, 9);
    regs[dr] = memory_read(mem, regs[Register::PC] + offset);
    update_flags(regs, dr);
}

// store
pub fn op_st(instr: u16, regs: &mut [u16], mem: &mut [u16]) {
    let sr = ((instr >> 9) & 0x7) as usize;
    let offset = sign_extend(instr & 0x1FF, 9);
    memory_write(mem, regs[Register::PC] + offset, regs[sr]);
}

// jump to subroutine
pub fn op_jsr(instr: u16, regs: &mut [u16]) {
    let is_long = (instr >> 11) & 0x1;
    regs[Register::R7] = regs[Register::PC];

    if is_long == 1 {
        let offset = sign_extend(instr & 0x7FF, 11);
        regs[Register::PC] += offset; // JSR
    } else {
        let base = ((instr >> 6) & 0x7) as usize;
        regs[Register::PC] = regs[base]; // JSRR
    }
}

// bitwise logical AND
pub fn op_and(instr: u16, regs: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let sr1 = ((instr >> 6) & 0x7) as usize;
    let is_imm = (instr >> 5) & 0x1;

    if is_imm == 1 {
        regs[dr] = regs[sr1] & sign_extend(instr & 0x1F, 5);
    } else {
        let sr2: usize = (instr & 0x7).into();
        regs[dr] = regs[sr1] & regs[sr2];
    }
    update_flags(regs, dr);
}

// load register (base + offset)
pub fn op_ldr(instr: u16, regs: &mut [u16], mem: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let base = ((instr >> 6) & 0x7) as usize;
    let offset = sign_extend(instr & 0x3F, 6);
    regs[dr] = memory_read(mem, regs[base] + offset);
    update_flags(regs, dr);
}

// store register (base + offset)
pub fn op_str(instr: u16, regs: &mut [u16], mem: &mut [u16]) {
    let sr = ((instr >> 9) & 0x7) as usize;
    let base = ((instr >> 6) & 0x7) as usize;
    let offset = sign_extend(instr & 0x3F, 6);
    memory_write(mem, regs[base] + offset, regs[sr]);
}

// bitwise complement
pub fn op_not(instr: u16, regs: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let sr = ((instr >> 6) & 0x7) as usize;
    regs[dr] = !regs[sr];
    update_flags(regs, dr);
}

// load indirect
pub fn op_ldi(instr: u16, regs: &mut [u16], mem: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let offset = sign_extend(instr & 0x1FF, 9);
    regs[Register::R0] = memory_read(mem, regs[Register::PC] + offset);
    update_flags(regs, dr);
}

// store indirect
pub fn op_sti(instr: u16, regs: &mut [u16], mem: &mut [u16]) {
    let sr = ((instr >> 9) & 0x7) as usize;
    let offset = sign_extend(instr & 0x1FF, 9);
    let indirect = memory_read(mem, regs[Register::PC] + offset);
    memory_write(mem, indirect, regs[sr]);
}

// jump / return from subroutine; this also handles RET
pub fn op_jmp(instr: u16, regs: &mut [u16]) {
    let addr = (instr >> 6) & 0x7F;
    regs[Register::PC] = regs[addr as usize];
}

// load effective address
pub fn op_lea(instr: u16, regs: &mut [u16]) {
    let dr = ((instr >> 9) & 0x7) as usize;
    let offset = sign_extend(instr & 0x1FF, 9);
    regs[dr] = regs[Register::PC] + offset;
    update_flags(regs, dr);
}

// sign extend value to n-bits
fn sign_extend(x: u16, bits: u8) -> u16 {
    if ((x >> (bits - 1)) & 1) == 1 {
        return x | (0xFFFF << bits);
    }
    return x;
}

// update condition flags
fn update_flags(regs: &mut [u16], r: usize) {
    regs[Register::COND] = {
        if regs[r] == 0 {
            Flag::ZERO
        } else if (regs[r] >> 15) == 1 {
            Flag::NEG
        } else {
            Flag::POS
        }
    } as u16;
}

// read value from memory address
pub fn memory_read(mem: &mut [u16], addr: u16) -> u16 {
    if addr == (MMRegister::KBSR as u16) {
        let mut buff = [0; 1];
        stdin().read_exact(&mut buff).unwrap();

        if buff[0] != 0 {
            mem[MMRegister::KBSR as usize] = 1 << 15;
            mem[MMRegister::KBDR as usize] = buff[0] as u16;
        } else {
            mem[MMRegister::KBSR as usize] = 0;
        }
    }
    return mem[addr as usize];
}

// write value to memory address
fn memory_write(mem: &mut [u16], addr: u16, val: u16) {
    mem[addr as usize] = val;
}
