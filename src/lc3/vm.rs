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
            OpCode::BR => self.op_br(instr),
            OpCode::ADD => self.op_add(instr),
            OpCode::LD => self.op_ld(instr),
            OpCode::ST => self.op_st(instr),
            OpCode::JSR => self.op_jsr(instr),
            OpCode::AND => self.op_and(instr),
            OpCode::LDR => self.op_ldr(instr),
            OpCode::STR => self.op_str(instr),
            OpCode::RTI => self.op_rti(),
            OpCode::NOT => self.op_not(instr),
            OpCode::LDI => self.op_ldi(instr),
            OpCode::STI => self.op_sti(instr),
            OpCode::JMP => self.op_jmp(instr),
            OpCode::RES => self.op_res(),
            OpCode::LEA => self.op_lea(instr),
            OpCode::TRAP => self.op_trap(instr),
        }
        // load one instruction from ram[pc]
        // pc++
        // determine instruction
        // perform instruction
    }

    // get current instruction
    fn get_curr_ins(&self) -> u16 {
        self.memory_read(self.regs[Register::PC])
    }

    // read value from memory address
    fn memory_read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    // write value to memory address
    fn memory_write(&mut self, addr: u16, val: u16) {
        self.mem[addr as usize] = val;
    }

    // update condition flags
    fn update_flags(&mut self, r: usize) {
        let test = self.regs[r];
        self.regs[Register::COND] = {
            if test == 0 {
                Flag::ZERO
            } else if (test >> 15) == 1 {
                Flag::NEG
            } else {
                Flag::POS
            }
        } as u16;
    }

    fn op_br(&mut self, instr: u16) {
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let cond = (instr >> 9) & 0x7F; // NZP

        if (cond & (self.regs[Register::COND])) > 0 {
            self.regs[Register::PC] += pc_offset;
        }
    }

    fn op_add(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let sr1 = ((instr >> 6) & 0x7) as usize;
        let is_imm = (instr >> 5) & 0x1;

        if is_imm == 1 {
            self.regs[dr] = self.regs[sr1] + sign_extend(instr & 0x1F, 5);
        } else {
            let sr2: usize = (instr & 0x7).into();
            self.regs[dr] = self.regs[sr1] + self.regs[sr2];
        }
        self.update_flags(dr);
    }

    fn op_ld(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let offset = sign_extend(instr & 0x1FF, 9);
        self.regs[dr] = self.memory_read(self.regs[Register::PC] + offset);
        self.update_flags(dr);
    }

    fn op_st(&mut self, instr: u16) {
        let sr = ((instr >> 9) & 0x7) as usize;
        let offset = sign_extend(instr & 0x1FF, 9);
        self.memory_write(self.regs[Register::PC] + offset, self.regs[sr]);
    }

    fn op_jsr(&mut self, instr: u16) {
        let is_long = (instr >> 11) & 0x1;
        self.regs[Register::R7] = self.regs[Register::PC];

        if is_long == 1 {
            let offset = sign_extend(instr & 0x7FF, 11);
            self.regs[Register::PC] += offset; // JSR
        } else {
            let base = ((instr >> 6) & 0x7) as usize;
            self.regs[Register::PC] = self.regs[base]; // JSRR
        }
    }

    fn op_and(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let sr1 = ((instr >> 6) & 0x7) as usize;
        let is_imm = (instr >> 5) & 0x1;

        if is_imm == 1 {
            self.regs[dr] = self.regs[sr1] & sign_extend(instr & 0x1F, 5);
        } else {
            let sr2: usize = (instr & 0x7).into();
            self.regs[dr] = self.regs[sr1] & self.regs[sr2];
        }
        self.update_flags(dr);
    }

    fn op_ldr(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let base = ((instr >> 6) & 0x7) as usize;
        let offset = sign_extend(instr & 0x3F, 6);
        self.regs[dr] = self.memory_read(self.regs[base] + offset);
        self.update_flags(dr);
    }

    fn op_str(&mut self, instr: u16) {
        let sr = ((instr >> 9) & 0x7) as usize;
        let base = ((instr >> 6) & 0x7) as usize;
        let offset = sign_extend(instr & 0x3F, 6);
        self.memory_write(self.regs[base] + offset, self.regs[sr]);
    }

    fn op_rti(&mut self) {
        unimplemented!()
    }

    fn op_not(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let sr = ((instr >> 6) & 0x7) as usize;
        self.regs[dr] = !self.regs[sr];
        self.update_flags(dr);
    }

    fn op_ldi(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let offset = sign_extend(instr & 0x1FF, 9);
        self.regs[Register::R0] = self.memory_read(self.regs[Register::PC] + offset);
        self.update_flags(dr);
    }

    fn op_sti(&mut self, instr: u16) {
        let sr = ((instr >> 9) & 0x7) as usize;
        let offset = sign_extend(instr & 0x1FF, 9);
        let indirect = self.memory_read(self.regs[Register::PC] + offset);
        self.memory_write(indirect, self.regs[sr]);
    }

    fn op_jmp(&mut self, instr: u16) {
        let addr = (instr >> 6) & 0x7F;
        self.regs[Register::PC] = self.regs[addr as usize];
        // note: this also handles RET
    }

    fn op_res(&mut self) {
        unimplemented!()
    }

    fn op_lea(&mut self, instr: u16) {
        let dr = ((instr >> 9) & 0x7) as usize;
        let offset = sign_extend(instr & 0x1FF, 9);
        self.regs[dr] = self.regs[Register::PC] + offset;
        self.update_flags(dr);
    }

    fn op_trap(&mut self, instr: u16) {
        match TrapVector::from_u16(instr & 0xFF) {
            TrapVector::GETC => {}
            TrapVector::OUT => {}
            TrapVector::PUTS => {}
            TrapVector::IN => {}
            TrapVector::PUTSP => {}
            TrapVector::HALT => {}
        }
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

pub enum TrapVector {
    GETC = 0x20,  //
    OUT = 0x21,   //
    PUTS = 0x22,  //
    IN = 0x23,    //
    PUTSP = 0x24, //
    HALT = 0x25,  //
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

// sign extend value to n-bits
fn sign_extend(x: u16, bits: u8) -> u16 {
    if ((x >> (bits - 1)) & 1) == 1 {
        return x | (0xFFFF << bits);
    }
    return x;
}
