use crate::cpu::cpu_context::{CpuContext, R8};

pub fn load_from(context: &mut CpuContext, opcode: u8) -> Result<(), String> {
    print!("ld ");
    let src_param = R8::get_r8_param(false, opcode, 0, context)?;
    let src = src_param.read(context)?;
    src_param.log();
    let dst_param = R8::get_r8_param(false, opcode, 3, context)?;
    dst_param.write(context, src)?;
    Ok(())
}
