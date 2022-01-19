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

// sign extend value to n-bits
fn sign_extend(u16 x, i8 bits) {
    if ((x >> (bits - 1)) & 1) {
       x |= (0xFFFF << bits); 
    }
    return x;
}
