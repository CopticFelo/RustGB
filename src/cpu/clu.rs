use crate::{
    cpu::{alu, clock::Clock, reg_file::RegFile},
    mem::map::MemoryMap,
};

pub struct CLU<'a> {
    registers: &'a mut RegFile,
    memory: &'a mut MemoryMap,
    clock: &'a mut Clock,
}

impl<'a> CLU<'a> {
    pub fn init(
        registers: &'a mut RegFile,
        memory: &'a mut MemoryMap,
        clock: &'a mut Clock,
    ) -> Self {
        Self {
            registers,
            memory,
            clock,
        }
    }

    fn fetch(&mut self) -> u8 {
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
            src = self
                .memory
                .read(alu::read_u16(&self.registers.l, &self.registers.h))?;
        } else {
            src = *self.registers.match_register(src)?;
        }
        let dst = alu::read_bits(opcode, 3, 3);
        if dst == 6 {
            self.memory
                .write(alu::read_u16(&self.registers.l, &self.registers.h), src)?;
        } else {
            *self.registers.match_register(dst)? = src;
        }
        Ok(())
    }

    fn cond_jmp(&mut self, opcode: u8, is_relative: bool) -> Result<(), String> {
        if self
            .registers
            .match_condition(alu::read_bits(opcode, 3, 2))?
        {
            if is_relative {
                let offset_address = self.registers.pc as i16 + self.fetch() as i16;
                self.registers.pc = offset_address as u16;
            } else {
                self.registers.pc = alu::read_u16(&self.fetch(), &self.fetch());
            }
        }
        Ok(())
    }

    pub fn start_exec_cycle(&mut self) -> Result<(), String> {
        loop {
            let opcode = self.fetch();
            println!("{:#X}", opcode);
            println!("{:#X}", self.registers.pc);
            match opcode {
                0x0 => (),                                                                           // NOP
                0xC3 | 0xD3 | 0xCA | 0xDA => self.cond_jmp(opcode, false)?, // JP cc, imm16
                0xC4 => self.registers.pc = alu::read_u16(&self.fetch(), &self.fetch()), // JP imm16
                0x20 | 0x30 | 0x28 | 0x38 => self.cond_jmp(opcode, true)?,  // JR cc, imm8
                0x18 => self.registers.pc = (self.registers.pc as i16 + self.fetch() as i16) as u16, // JR imm8
                0xE9 => self.registers.pc = alu::read_u16(&self.registers.l, &self.registers.h), // JP hl
                0x40..0x80 => self.load_from(opcode)?, // LD r8, r8 | LD r8, [hl] | LD [hl], r8
                _ => eprintln!("Unimplemented"),
            }
        }
    }
}
