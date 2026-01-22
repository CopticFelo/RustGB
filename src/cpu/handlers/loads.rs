use crate::cpu::{cpu_context::CpuContext, operands::R8};

pub fn load_from(context: &mut CpuContext, opcode: u8) -> Result<(), String> {
    print!("ld ");
    let src_param = R8::get_r8_param(false, opcode, 0, context)?;
    let src = src_param.read(context)?;
    let dst_param = R8::get_r8_param(false, opcode, 3, context)?;
    dst_param.log();
    src_param.log();
    dst_param.write(context, src)?;
    Ok(())
}
