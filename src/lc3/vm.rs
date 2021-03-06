use super::opcodes::*;
use super::registers::Register;
use super::traps::*;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{BufReader, Result as IoResult};
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
        let mut regs: [u16; 10] = [0; 10];
        regs[Register::PC] = PC_START;

        Self {
            mem: [0; MEM_SIZE],
            regs: regs,
            is_halted: false,
        }
    }

    // load VM with LC3 program image
    pub fn load_img(&mut self, img_path: &Path) -> IoResult<()> {
        let f = File::open(img_path).expect("File not found");
        let mut buff = BufReader::new(f);

        let org = buff.read_u16::<BigEndian>().expect("bad starting address");
        let mut addr = org as usize;
        loop {
            match buff.read_u16::<BigEndian>() {
                Ok(instr) => {
                    self.mem[addr as usize] = instr;
                    addr += 1;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::UnexpectedEof {
                        println!("Error loading program: {}", e);
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    // run VM until halted
    pub fn run(&mut self) {
        while !self.is_halted {
            self.cycle();

            if self.regs[Register::PC] >= MEM_SIZE as u16 {
                self.is_halted = true;
            }
        }
    }

    // perform one CPU cycle
    pub fn cycle(&mut self) {
        let instr = memory_read(&mut self.mem, self.regs[Register::PC]);
        let op = OpCode::from_u16(instr >> 12);
        self.regs[Register::PC] += 1;

        match op {
            OpCode::BR => op_br(instr, &mut self.regs),
            OpCode::ADD => op_add(instr, &mut self.regs),
            OpCode::LD => op_ld(instr, &mut self.regs, &mut self.mem),
            OpCode::ST => op_st(instr, &mut self.regs, &mut self.mem),
            OpCode::JSR => op_jsr(instr, &mut self.regs),
            OpCode::AND => op_and(instr, &mut self.regs),
            OpCode::LDR => op_ldr(instr, &mut self.regs, &mut self.mem),
            OpCode::STR => op_str(instr, &mut self.regs, &mut self.mem),
            OpCode::RTI => unimplemented!(),
            OpCode::NOT => op_not(instr, &mut self.regs),
            OpCode::LDI => op_ldi(instr, &mut self.regs, &mut self.mem),
            OpCode::STI => op_sti(instr, &mut self.regs, &mut self.mem),
            OpCode::JMP => op_jmp(instr, &mut self.regs),
            OpCode::RES => unimplemented!(),
            OpCode::LEA => op_lea(instr, &mut self.regs),
            OpCode::TRAP => self.trap(instr),
        }
    }

    // handle trap vector
    fn trap(&mut self, instr: u16) {
        match TrapVector::from_u16(instr & 0xFF) {
            TrapVector::GETC => trap_getc(&mut self.regs),
            TrapVector::OUT => trap_out(&mut self.regs),
            TrapVector::PUTS => trap_puts(self.regs[Register::R0], &mut self.mem),
            TrapVector::IN => trap_in(&mut self.regs),
            TrapVector::PUTSP => trap_putsp(&mut self.regs, &mut self.mem),
            TrapVector::HALT => {
                print!("HALT");
                self.is_halted = true;
            }
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
