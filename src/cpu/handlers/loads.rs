use crate::cpu::{
    alu,
    cpu_context::CpuContext,
    operands::{R8, R16, R16Type},
};

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

pub fn load16(context: &mut CpuContext, opcode: u8) -> Result<(), String> {
    let param = R16::new(opcode, 4, R16Type::R16)?;
    param.write(
        alu::read_u16(&context.fetch(), &context.fetch()),
        &mut context.registers,
    );
    print!("ld r16 imm16");
    Ok(())
}
