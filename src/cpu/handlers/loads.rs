use crate::cpu::cpu_context::{CpuContext, R8};

pub fn load_from(context: &mut CpuContext, opcode: u8) -> Result<(), String> {
    print!("ld ");
    let src_param = R8::get_r8_param(false, opcode, 0, context)?;
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
    let dst_param = R8::get_r8_param(false, opcode, 3, context)?;
    match dst_param {
        // IMPORTANT: because get_r8_param() alr calls clock.tick() when reading the value,
        // you don't need to call clock.tick again when writing as this part doesn't even
        // need a cycle for reading but only one for writing, so writing here is free
        // TODO: Fix this mess
        R8::Hl { addr, value: _ } => {
            print!("[hl] ");
            context.memory.write(addr, src)?
        }
        R8::Register { reg, value: _ } => {
            print!("r8 ");
            *context.registers.match_r8(reg)? = src;
        }
        R8::N8(n) => return Err(format!("invalid dst in ld instruction {}", n)),
    };
    println!("{}", src_str);
    Ok(())
}
