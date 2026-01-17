use crate::{
    cpu::{alu::alu, clock::Clock, reg_file::RegFile},
    mem::map::MemoryMap,
};

// the r8 param is a 3 bit param in the instruction opcode
// it represents an 8-bit register
// or the memory value (8-bit) pointed to by the 16-bit hl register
// from 0-7 in order (b,c,d,e,h,l,[hl],a)
pub enum R8 {
    Register { reg: u8, value: u8 },
    Hl { addr: u16, value: u8 },
    N8(u8), // this is added for convinience some instructions that take r8 have an identical
            // version that takes imm8 (i.e the next byte on the rom)
}

impl R8 {
    pub fn get_r8_param(n8: bool, opcode: u8, index: u8, clu: &mut CLU) -> Result<Self, String> {
        if n8 {
            return Ok(Self::N8(clu.fetch()));
        }
        let param = alu::read_bits(opcode, index, 3);
        if param == 6 {
            let addr = alu::read_u16(&clu.registers.l, &clu.registers.h);
            clu.clock.tick();
            Ok(Self::Hl {
                addr,
                value: clu.memory.read(addr)?,
            })
        } else {
            Ok(Self::Register {
                reg: param,
                value: *clu.registers.match_r8(param)?,
            })
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
        print!("ld ");
        let src_param = R8::get_r8_param(false, opcode, 0, self)?;
        let src_str;
        let src = match src_param {
            R8::Register { reg: _, value } => {
                src_str = "r8";
                value
            }
            R8::Hl { addr: _, value } => {
                src_str = "[hl]";
                value
            }
            _ => return Err("invalid src in ld instruction: n8".to_string()),
        };
        let dst_param = R8::get_r8_param(false, opcode, 3, self)?;
        match dst_param {
            // IMPORTANT: because get_r8_param() alr calls clock.tick() when reading the value,
            // you don't need to call clock.tick again when writing as this part doesn't even
            // need a cycle for reading but only one for writing, so writing here is free
            // TODO: Fix this mess
            R8::Hl { addr, value: _ } => {
                print!("[hl] ");
                self.memory.write(addr, src)?
            }
            R8::Register { reg, value: _ } => {
                print!("r8 ");
                *self.registers.match_r8(reg)? = src;
            }
            R8::N8(n) => return Err(format!("invalid dst in ld instruction {}", n)),
        };
        println!("{}", src_str);
        Ok(())
    }

    fn jmp(&mut self, opcode: u8, is_relative: bool) -> Result<(), String> {
        let target_address: u16;
        let is_conditional: bool;
        if is_relative {
            print!("jr ");
            is_conditional = opcode != 0x18;
            target_address = (self.registers.pc as i16 + self.fetch() as i16) as u16;
        } else {
            print!("jp ");
            is_conditional = opcode != 0xC3;
            target_address = alu::read_u16(&self.fetch(), &self.fetch());
        }
        if is_conditional {
            print!("cc ");
        }
        println!("n16");
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
            print!("{:#X}: ", self.registers.pc);
            print!("{:#X} -> ", opcode);
            match opcode {
                0x0 => println!("nop"),                                       // NOP
                0xC2 | 0xD2 | 0xCA | 0xDA | 0xC3 => self.jmp(opcode, false)?, // JP cc, imm16 | JP imm16
                0x20 | 0x30 | 0x28 | 0x38 | 0x18 => self.jmp(opcode, true)?, // JR cc, imm8 | JR imm8
                0xE9 => {
                    println!("jp [hl]");
                    self.registers.pc = alu::read_u16(&self.registers.l, &self.registers.h);
                } // JP hl
                0x40..0x80 => self.load_from(opcode)?, // LD r8, r8 | LD r8, [hl] | LD [hl], r8
                0x80..0x90 | 0xC6 | 0xCE => alu::add(opcode, self)?, // ADD/ADC A, r8 | ADD/ADC A, [hl] | ADD/ADC A, imm8
                0x90..0xA0 | 0xD6 | 0xDE => alu::sub(opcode, self)?, // SUB/SBC A, r8 | SUB/SBC A, [hl] | SUB/SBC A, imm8
                0xA0..0xA8 | 0xE6 => alu::and(opcode, self)?, // AND A, r8 | AND A, [hl] | AND A, imm8
                0xA8..0xB0 | 0xEE => alu::xor(opcode, self)?, // XOR A, r8 | XOR A, [hl] | XOR A, imm8
                0xB0..0xB8 | 0xF6 => alu::or(opcode, self)?,  // OR A, r8 | OR A, [hl] | OR A, imm8
                0xB8..0xC0 | 0xFE => alu::cp(opcode, self)?,  // CP A, r8 | CP A, [hl] | CP A, imm8
                0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => {
                    alu::inc_r8(opcode, self, 1)?
                } // INC r8, INC [hl]
                0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => {
                    alu::inc_r8(opcode, self, -1)?
                } // DEC r8, DEC [hl]
                0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB..0xEE | 0xF4 | 0xFC | 0xFD => {
                    return Err(format!("Illegal operation {opcode}"));
                }
                _ => println!("<unsupported>"),
            }
        }
    }
}
