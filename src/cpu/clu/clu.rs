use crate::{
    cpu::{alu::alu, clock::Clock, reg_file::RegFile},
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
    pub fn get_r8_param(n8: bool, opcode: u8, index: u8, clu: &mut CLU) -> Self {
        if n8 {
            return Self::N8(clu.fetch());
        }
        let param = alu::read_bits(opcode, index, 3);
        if param == 6 {
            Self::Hl(alu::read_u16(&clu.registers.l, &clu.registers.h))
        } else {
            Self::Register(param)
        }
    }
}

pub struct CLU {
    pub registers: RegFile,
    pub memory: MemoryMap,
    pub clock: Clock,
}

impl CLU {
    pub fn init(registers: RegFile, memory: MemoryMap, clock: Clock) -> Self {
        Self {
            registers,
            memory,
            clock,
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let result = match self.memory.read(self.registers.pc) {
            Ok(op) => op,
            // HACK: Probably improper error handling
            Err(s) => {
                println!("{}", s);
                0x0
            }
        };
        self.registers.pc += 1;
        self.clock.tick();
        result
    }

    fn load_from(&mut self, opcode: u8) -> Result<(), String> {
        let mut src = alu::read_bits(opcode, 0, 3);
        if src == 6 {
            self.clock.tick();
            src = self
                .memory
                .read(alu::read_u16(&self.registers.l, &self.registers.h))?;
        } else {
            src = *self.registers.match_register(src)?;
        }
        let dst = alu::read_bits(opcode, 3, 3);
        if dst == 6 {
            self.clock.tick();
            self.memory
                .write(alu::read_u16(&self.registers.l, &self.registers.h), src)?;
        } else {
            *self.registers.match_register(dst)? = src;
        }
        Ok(())
    }

    fn jmp(&mut self, opcode: u8, is_relative: bool) -> Result<(), String> {
        let target_address: u16;
        let is_conditional: bool;
        if is_relative {
            is_conditional = opcode != 0x18;
            target_address = (self.registers.pc as i16 + self.fetch() as i16) as u16;
        } else {
            is_conditional = opcode != 0xC3;
            target_address = alu::read_u16(&self.fetch(), &self.fetch());
        }
        if self
            .registers
            .match_condition(alu::read_bits(opcode, 3, 2))?
            || !is_conditional
        {
            self.registers.pc = target_address;
            self.clock.tick();
        }
        Ok(())
    }

    pub fn start_exec_cycle(&mut self) -> Result<(), String> {
        loop {
            let opcode = self.fetch();
            println!("{:#X}", opcode);
            println!("{:#X}", self.registers.pc);
            match opcode {
                0x0 => (),                                                                       // NOP
                0xC2 | 0xD2 | 0xCA | 0xDA | 0xC3 => self.jmp(opcode, false)?, // JP cc, imm16 | JP imm16
                0x20 | 0x30 | 0x28 | 0x38 | 0x18 => self.jmp(opcode, true)?, // JR cc, imm8 | JR imm8
                0xE9 => self.registers.pc = alu::read_u16(&self.registers.l, &self.registers.h), // JP hl
                0x40..0x80 => self.load_from(opcode)?, // LD r8, r8 | LD r8, [hl] | LD [hl], r8
                0x80..0x90 | 0xC6 | 0xCE => alu::add(opcode, self)?, // ADD/ADC A, r8 | ADD/ADC A, [hl] | ADD/ADC A, imm8
                0x90..0xA0 | 0xD6 | 0xDE => alu::sub(opcode, self)?, // SUB/SBC A, r8 | SUB/SBC A, [hl] | SUB/SBC A, imm8
                0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB..0xEF | 0xF4 | 0xFC | 0xFD => {
                    return Err(format!("Illegal operation {opcode}"));
                }
                _ => eprintln!("Unimplemented"),
            }
        }
    }
}
