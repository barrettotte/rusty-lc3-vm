use super::registers::Register;
use std::io::Read;

#[derive(Debug)]
pub enum TrapVector {
    GETC = 0x20,
    OUT = 0x21,
    PUTS = 0x22,
    IN = 0x23,
    PUTSP = 0x24,
    HALT = 0x25,
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
    regs[Register::R0] = buff[0].into();
}

// output a character
pub fn trap_out(regs: &mut [u16]) {
    print!("{}", (regs[Register::R0] as u8) as char);
}

// output word string
pub fn trap_puts(r0: u16, memory: &mut [u16]) {
    let mut idx: usize = r0 as usize;

    while idx < memory.len() {
        print!("{}", (memory[idx] as u8) as char);
        idx += 1;
    }
}

// get character from keyboard, echoed onto terminal
pub fn trap_in(regs: &mut [u16]) {
    let mut buff = [0 as u8; 1];

    print!("Enter a character: ");
    std::io::stdin().read_exact(&mut buff).unwrap();
    regs[Register::R0] = buff[0].into();
}

// output byte string
pub fn trap_putsp(regs: &mut [u16], memory: &mut [u16]) {
    let mut idx = regs[Register::R0] as usize;

    while idx < memory.len() && memory[idx] != 0 {
        let bytes = (memory[idx] as u16).to_be_bytes();
        print!("{}", bytes[1] as char);

        if bytes[0] != 0 {
            print!("{}", bytes[0] as char);
        }
        idx += 1;
    }
}
