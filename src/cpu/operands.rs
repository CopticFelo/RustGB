use crate::cpu::{alu, cpu_context::CpuContext};

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
            Self::Register(_) => print!("r8 "),
            Self::Hl(_) => print!("[hl] "),
            Self::N8(_) => print!("imm8 "),
        }
    }
}
