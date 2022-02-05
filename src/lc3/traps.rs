use super::registers::Register;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum TrapVector {
    GETC = 0x20,  // get character from keyboard, not echoed onto terminal
    OUT = 0x21,   // output a character
    PUTS = 0x22,  // output word string
    IN = 0x23,    // get character from keyboard, echoed onto terminal
    PUTSP = 0x24, // output byte string
    HALT = 0x25,  // halt processor
}

impl TrapVector {
    pub fn from_u16(value: u16) -> TrapVector {
        match value {
            0x20 => TrapVector::GETC,
            0x21 => TrapVector::OUT,
            0x22 => TrapVector::PUTS,
            0x23 => TrapVector::IN,
            0x24 => TrapVector::PUTSP,
            0x25 => TrapVector::HALT,
            _ => panic!("Unknown trap vector: {}", value),
        }
    }
}

// get character from keyboard, not echoed onto terminal
pub fn trap_getc(regs: &mut [u16]) {
    let mut buff = [0 as u8; 1];
    std::io::stdin().read_exact(&mut buff).unwrap();
    regs[Register::R0] = buff[0] as u16;
}

// output a character
pub fn trap_out(regs: &mut [u16]) {
    print!("{}", (regs[Register::R0] as u8) as char);
}

// output word string
pub fn trap_puts(r0: u16, memory: &mut [u16]) {
    let mut idx: usize = r0 as usize;
    let mut word = memory[idx];

    while word != 0x0000 {
        print!("{}", (word as u8) as char);
        idx += 1;
        word = memory[idx];
    }
    std::io::stdout().flush().expect("failed to flush");
}

// get character from keyboard, echoed onto terminal
pub fn trap_in(regs: &mut [u16]) {
    let mut buff = [0 as u8; 1];
    print!("Enter a character: ");
    std::io::stdout().flush().expect("failed to flush");
    std::io::stdin().read_exact(&mut buff).unwrap();
    regs[Register::R0] = buff[0] as u16;
}

// output byte string
pub fn trap_putsp(regs: &mut [u16], memory: &mut [u16]) {
    let mut idx = regs[Register::R0] as usize;
    let mut word = memory[idx];

    while idx != 0x0000 {
        let low = ((word & 0xFF) as u8) as char;
        let high = ((word >> 8) as u8) as char;

        print!("{}", low);
        if high != '\0' {
            print!("{}", high);
        }
        idx += 1;
        word = memory[idx];
    }
    std::io::stdout().flush().expect("failed to flush");
}
