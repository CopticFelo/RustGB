use crate::{
    cpu::{clock::Clock, reg_file::RegFile},
    mem::map::MemoryMap,
};

pub struct CLU<'a> {
    registers: &'a mut RegFile,
    memory: &'a MemoryMap,
    clock: &'a mut Clock,
}

impl<'a> CLU<'a> {
    pub fn init(registers: &'a mut RegFile, memory: &'a MemoryMap, clock: &'a mut Clock) -> Self {
        Self {
            registers,
            memory,
            clock,
        }
    }
    fn fetch(&mut self) -> u8 {
        let result = match self.memory.read(self.registers.pc) {
            Ok(op) => op,
            Err(s) => {
                println!("{}", s);
                0x0
            }
        };
        self.registers.pc += 1;
        self.clock.tick();
        result
    }

    pub fn start_exec_cycle(&mut self) {
        let opcode = self.fetch();
        match opcode {
            0x0 => (),
            0xC3 => self.registers.pc = RegFile::read_u16(&self.fetch(), &self.fetch()),
            _ => eprintln!("Unimplemented"),
        }
    }
}
