use crate::cpu::{alu::alu, cpu_context::CpuContext};

pub fn jmp(clu: &mut CpuContext, opcode: u8, is_relative: bool) -> Result<(), String> {
    let target_address: u16;
    let is_conditional: bool;
    if is_relative {
        print!("jr ");
        is_conditional = opcode != 0x18;
        target_address = (clu.registers.pc as i16 + clu.fetch() as i16) as u16;
    } else {
        print!("jp ");
        is_conditional = opcode != 0xC3;
        target_address = alu::read_u16(&clu.fetch(), &clu.fetch());
    }
    if is_conditional {
        print!("cc ");
    }
    println!("n16");
    if clu
        .registers
        .match_condition(alu::read_bits(opcode, 3, 2))?
        || !is_conditional
    {
        clu.registers.pc = target_address;
        clu.clock.tick();
    }
    Ok(())
}
