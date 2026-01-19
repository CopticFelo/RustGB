use crate::{
    cpu::{alu::alu, clock::Clock, handlers::*, reg_file::RegFile},
    mem::map::MemoryMap,
};

// the r8 param is a 3 bit param in the instruction opcode
// it represents an 8-bit register
// or the memory value (8-bit) pointed to by the 16-bit hl register
// from 0-7 in order (b,c,d,e,h,l,[hl],a)
pub enum R8 {
    Register(u8),
    Hl(u16),
    N8(u8), // this is added for convinience some instructions that take r8 have an identical
            // version that takes imm8 (i.e the next byte on the rom)
}

impl R8 {
    pub fn get_r8_param(
        n8: bool,
        opcode: u8,
        index: u8,
        context: &mut CpuContext,
    ) -> Result<Self, String> {
        if n8 {
            return Ok(Self::N8(context.fetch()));
        }
        let param = alu::read_bits(opcode, index, 3);
        if param == 6 {
            let addr = alu::read_u16(&context.registers.l, &context.registers.h);
            Ok(Self::Hl(addr))
        } else {
            Ok(Self::Register(param))
        }
    }

    pub fn read(&self, context: &mut CpuContext) -> Result<u8, String> {
        match self {
            Self::Register(reg) => Ok(*context.registers.match_r8(*reg)?),
            Self::Hl(addr) => Ok(context.memory.read(&mut context.clock, *addr)?),
            Self::N8(n) => Ok(*n),
        }
    }

    pub fn write(&self, context: &mut CpuContext, value: u8) -> Result<(), String> {
        match self {
            Self::Register(reg) => {
                *context.registers.match_r8(*reg)? = value;
                Ok(())
            }
            Self::Hl(addr) => {
                context.memory.write(&mut context.clock, *addr, value)?;
                Ok(())
            }
            Self::N8(_) => Ok(()),
        }
    }

    //TODO: implement a better logging system
    pub fn log(&self) {
        match self {
            Self::Register(_) => print!("r8"),
            Self::Hl(_) => print!("[hl]"),
            Self::N8(_) => print!("imm8"),
        }
    }
}

pub struct CpuContext {
    pub registers: RegFile,
    pub memory: MemoryMap,
    pub clock: Clock,
}

impl CpuContext {
    pub fn init(registers: RegFile, memory: MemoryMap, clock: Clock) -> Self {
        Self {
            registers,
            memory,
            clock,
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let result = match self.memory.read(&mut self.clock, self.registers.pc) {
            Ok(op) => op,
            // HACK: Probably improper error handling
            Err(s) => {
                println!("{}", s);
                0x0
            }
        };
        self.registers.pc += 1;
        result
    }

    pub fn start_exec_cycle(&mut self) -> Result<(), String> {
        loop {
            let opcode = self.fetch();
            print!("{:#X}: ", self.registers.pc);
            print!("{:#X} -> ", opcode);
            match opcode {
                0x0 => print!("nop"),                                                 // NOP
                0xC2 | 0xD2 | 0xCA | 0xDA | 0xC3 => jumps::jmp(self, opcode, false)?, // JP cc, imm16 | JP imm16
                0x20 | 0x30 | 0x28 | 0x38 | 0x18 => jumps::jmp(self, opcode, true)?, // JR cc, imm8 | JR imm8
                0xE9 => {
                    println!("jp [hl]");
                    self.registers.pc = alu::read_u16(&self.registers.l, &self.registers.h);
                } // JP hl
                0x40..0x80 => loads::load_from(self, opcode)?, // LD r8, r8 | LD r8, [hl] | LD [hl], r8
                0x80..0x90 | 0xC6 | 0xCE => arithmetic::add(opcode, self)?, // ADD/ADC A, r8 | ADD/ADC A, [hl] | ADD/ADC A, imm8
                0x90..0xA0 | 0xD6 | 0xDE => arithmetic::sub(opcode, self)?, // SUB/SBC A, r8 | SUB/SBC A, [hl] | SUB/SBC A, imm8
                0xA0..0xA8 | 0xE6 => arithmetic::and(opcode, self)?, // AND A, r8 | AND A, [hl] | AND A, imm8
                0xA8..0xB0 | 0xEE => arithmetic::xor(opcode, self)?, // XOR A, r8 | XOR A, [hl] | XOR A, imm8
                0xB0..0xB8 | 0xF6 => arithmetic::or(opcode, self)?, // OR A, r8 | OR A, [hl] | OR A, imm8
                0xB8..0xC0 | 0xFE => arithmetic::cp(opcode, self)?, // CP A, r8 | CP A, [hl] | CP A, imm8
                0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => {
                    arithmetic::inc_r8(opcode, self, 1)?
                } // INC r8, INC [hl]
                0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => {
                    arithmetic::inc_r8(opcode, self, -1)?
                } // DEC r8, DEC [hl]
                0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB..0xEE | 0xF4 | 0xFC | 0xFD => {
                    return Err(format!("Illegal operation {opcode}"));
                }
                _ => print!("<unsupported>"),
            }
            println!();
        }
    }
}
