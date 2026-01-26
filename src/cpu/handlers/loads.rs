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

pub fn load_r16mem_a(opcode: u8, context: &mut CpuContext) -> Result<(), String> {
    let param = R16::new(opcode, 4, R16Type::R16Mem)?;
    let addr = param.read(&context.registers);
    context
        .memory
        .write(&mut context.clock, addr, context.registers.a)?;
    let raw_param = alu::read_bits(opcode, 4, 2);
    if raw_param == 0x2 {
        param.write(addr + 1, &mut context.registers);
    } else if raw_param == 0x3 {
        param.write(addr - 1, &mut context.registers);
    }
    print!("ld [r16mem] a");
    Ok(())
}

pub fn load_a_r16mem(opcode: u8, context: &mut CpuContext) -> Result<(), String> {
    let param = R16::new(opcode, 4, R16Type::R16Mem)?;
    let addr = param.read(&context.registers);
    let value = context.memory.read(&mut context.clock, addr)?;
    context.registers.a = value;
    let raw_param = alu::read_bits(opcode, 4, 2);
    if raw_param == 0x2 {
        param.write(addr + 1, &mut context.registers);
    } else if raw_param == 0x3 {
        param.write(addr - 1, &mut context.registers);
    }
    print!("ld a [r16mem]");
    Ok(())
}
