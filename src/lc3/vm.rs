use super::cpu::{Cpu, Register};

const PC_START: u16 = 0x3000;

pub struct VirtualMachine {
    cpu: Cpu,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let cpu = Cpu::new();

        let regs = cpu.regs()[Register::PC] = 4;

        Self { cpu }
    }

    pub fn run() {}
}
