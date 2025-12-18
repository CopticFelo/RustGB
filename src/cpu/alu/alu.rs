use crate::cpu::{
    clu::clu::{
        CLU,
        R8::{self, *},
    },
    reg_file::Flag,
};

pub fn read_u16(lo: &u8, hi: &u8) -> u16 {
    (*hi as u16) << 8 | *lo as u16
}

pub fn write_u16(lo: &mut u8, hi: &mut u8, value: u16) {
    *hi = (value >> 8) as u8;
    *lo = value as u8;
}

pub fn read_bits(num: u8, index: u8, length: u8) -> u8 {
    let mut out = 0;
    let mut index = index;
    for i in 0..length {
        out += ((num >> index) & 1) * 2_u8.pow(i as u32);
        index += 1;
    }
    out
}

pub fn write_bits(target: &mut u8, index: u8, length: u8, bits: u8) -> Result<(), String> {
    if index + length > 8 {
        return Err(format!(
            "Error: Trying to insert {length} bits at index {index} (Overflow)"
        ));
    }
    let mask: u8 = ((1 << length) - 1) << index;
    *target = (*target & !mask) | (bits << index);

    Ok(())
}

pub fn add(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    let mut src = read_bits(opcode, 0, 3);
    if opcode == 0xC6 || opcode == 0xCE {
        src = clu.fetch();
    } else if src == 6 {
        clu.clock.tick();
        src = clu
            .memory
            .read(read_u16(&clu.registers.l, &clu.registers.h))?;
    } else {
        src = *clu.registers.match_register(src)?;
    }
    let addend = if read_bits(opcode, 3, 1) == 1 && clu.registers.read_flag(Flag::Carry) {
        src + 1
    } else {
        src
    };
    let half_carry = (clu.registers.a & 0xF) + (addend & 0xF) > 0xF;
    let (res, carry) = clu.registers.a.overflowing_add(addend);
    let zero = res == 0;
    //FIX: Use set_all_flags()
    clu.registers.set_flag(Flag::HalfCarry, Some(half_carry))?;
    clu.registers.set_flag(Flag::Carry, Some(carry))?;
    clu.registers.set_flag(Flag::Zero, Some(zero))?;
    clu.registers.set_flag(Flag::Subtract, Some(false))?;
    clu.registers.a = res;
    Ok(())
}

pub fn sub(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    // FIX: This Code block right here is repeated 3 times so far
    let mut src = read_bits(opcode, 0, 3);
    if opcode == 0xD6 || opcode == 0xDE {
        src = clu.fetch();
    } else if src == 6 {
        clu.clock.tick();
        src = clu
            .memory
            .read(read_u16(&clu.registers.l, &clu.registers.h))?;
    } else {
        src = *clu.registers.match_register(src)?;
    }
    let subtrahend = if read_bits(opcode, 3, 1) == 1 && clu.registers.read_flag(Flag::Carry) {
        src + 1
    } else {
        src
    };
    let half_carry = (clu.registers.a & 0xF) < (subtrahend & 0xF);
    let (res, carry) = clu.registers.a.overflowing_sub(subtrahend);
    let zero = res == 0;
    //FIX: Use set_all_flags()
    clu.registers.set_flag(Flag::HalfCarry, Some(half_carry))?;
    clu.registers.set_flag(Flag::Carry, Some(carry))?;
    clu.registers.set_flag(Flag::Zero, Some(zero))?;
    clu.registers.set_flag(Flag::Subtract, Some(true))?;
    clu.registers.a = res;
    Ok(())
}

//NOTE: Untested
pub fn and(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    let r8_param = R8::get_r8_param(opcode == 0xE6, opcode, 0, clu)?;
    let src = match r8_param {
        Register { reg: _, value } | R8::Hl { addr: _, value } => value,
        N8(n) => n,
    };
    clu.registers.a &= src;
    clu.registers
        .set_all_flags(&[(clu.registers.a == 0) as u8, 0, 1, 0])?;
    Ok(())
}

//NOTE: Untested
pub fn xor(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    let r8_param = R8::get_r8_param(opcode == 0xEE, opcode, 0, clu)?;
    let src = match r8_param {
        Register { reg: _, value } | R8::Hl { addr: _, value } => value,
        N8(n) => n,
    };
    clu.registers.a ^= src;
    clu.registers
        .set_all_flags(&[(clu.registers.a == 0) as u8, 0, 0, 0])?;
    Ok(())
}

//NOTE: Untested
pub fn or(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    let r8_param = R8::get_r8_param(opcode == 0xF6, opcode, 0, clu)?;
    let src = match r8_param {
        Register { reg: _, value } | R8::Hl { addr: _, value } => value,
        N8(n) => n,
    };
    clu.registers.a |= src;
    clu.registers
        .set_all_flags(&[(clu.registers.a == 0) as u8, 0, 0, 0])?;
    Ok(())
}

//NOTE: Untested
pub fn cp(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    //NOTE: This code is also valid for sub, probably need to do that as well there
    let r8_param = R8::get_r8_param(opcode == 0xFE, opcode, 0, clu)?;
    let subtrahend = match r8_param {
        Register { reg: _, value } | R8::Hl { addr: _, value } => value,
        N8(n) => n,
    };
    let half_carry = (clu.registers.a & 0xF) < (subtrahend & 0xF);
    let (res, carry) = clu.registers.a.overflowing_sub(subtrahend);
    let zero = res == 0;
    clu.registers
        .set_all_flags(&[zero as u8, 1, half_carry as u8, carry as u8])?;
    Ok(())
}
