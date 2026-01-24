use crate::cpu::{alu, cpu_context::CpuContext, operands::R8};

pub fn load8(context: &mut CpuContext, opcode: u8) -> Result<(), String> {
    print!("ld ");
    let src_param = R8::get_r8_param(alu::read_bits(opcode, 6, 1) == 0, opcode, 0, context)?;
    let src = src_param.read(context)?;
    let dst_param = R8::get_r8_param(false, opcode, 3, context)?;
    dst_param.log();
    src_param.log();
    dst_param.write(context, src)?;
    Ok(())
}

pub fn load16(context: &mut CpuContext, opcode: u8) -> Result<(), &str> {
    let param = alu::read_bits(opcode, 4, 2);
    match param {
        0..=0x2 => {
            let value = alu::read_u16(&context.fetch(), &context.fetch());
            let reg_tuple = context.registers.match_r16(param)?;
            alu::write_u16(reg_tuple.1, reg_tuple.0, value);
        }
        0x3 => context.registers.sp = alu::read_u16(&context.fetch(), &context.fetch()),
        _ => return Err("Invalid r16 param"),
    }
    print!("ld r16 imm16");
    Ok(())
}
