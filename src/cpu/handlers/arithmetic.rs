use crate::cpu::{
    alu::alu::*,
    clu::clu::{
        CLU,
        R8::{self, *},
    },
    reg_file::Flag,
};

pub fn add(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    let mut src = read_bits(opcode, 0, 3);
    let operand_str;
    if opcode == 0xC6 || opcode == 0xCE {
        src = clu.fetch();
        operand_str = "imm8";
    } else if src == 6 {
        clu.clock.tick();
        src = clu
            .memory
            .read(read_u16(&clu.registers.l, &clu.registers.h))?;
        operand_str = "[hl]";
    } else {
        src = *clu.registers.match_r8(src)?;
        operand_str = "r8";
    }
    let addend = if read_bits(opcode, 3, 1) == 1 && clu.registers.read_flag(Flag::Carry) {
        print!("adc ");
        src + 1
    } else {
        print!("add ");
        src
    };
    println!("{}", operand_str);
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
    let operand_str;
    if opcode == 0xD6 || opcode == 0xDE {
        src = clu.fetch();
        operand_str = "imm8";
    } else if src == 6 {
        clu.clock.tick();
        src = clu
            .memory
            .read(read_u16(&clu.registers.l, &clu.registers.h))?;
        operand_str = "[hl]";
    } else {
        src = *clu.registers.match_r8(src)?;
        operand_str = "r8";
    }
    let subtrahend = if read_bits(opcode, 3, 1) == 1 && clu.registers.read_flag(Flag::Carry) {
        print!("sbc ");
        src + 1
    } else {
        print!("sub ");
        src
    };
    println!("{}", operand_str);
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
    print!("and ");
    let r8_param = R8::get_r8_param(opcode == 0xE6, opcode, 0, clu)?;
    let src = match r8_param {
        Register { reg: _, value } => {
            println!("r8");
            value
        }
        R8::Hl { addr: _, value } => {
            println!("[hl]");
            value
        }
        N8(n) => {
            println!("imm8");
            n
        }
    };
    clu.registers.a &= src;
    clu.registers
        .set_all_flags(&[(clu.registers.a == 0) as u8, 0, 1, 0])?;
    Ok(())
}

//NOTE: Untested
pub fn xor(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    print!("xor ");
    let r8_param = R8::get_r8_param(opcode == 0xEE, opcode, 0, clu)?;
    let src = match r8_param {
        Register { reg: _, value } => {
            println!("r8");
            value
        }
        R8::Hl { addr: _, value } => {
            println!("[hl]");
            value
        }
        N8(n) => {
            println!("imm8");
            n
        }
    };
    clu.registers.a ^= src;
    clu.registers
        .set_all_flags(&[(clu.registers.a == 0) as u8, 0, 0, 0])?;
    Ok(())
}

//NOTE: Untested
pub fn or(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    print!("or ");
    let r8_param = R8::get_r8_param(opcode == 0xF6, opcode, 0, clu)?;
    let src = match r8_param {
        Register { reg: _, value } => {
            println!("r8");
            value
        }
        R8::Hl { addr: _, value } => {
            println!("[hl]");
            value
        }
        N8(n) => {
            println!("imm8");
            n
        }
    };
    clu.registers.a |= src;
    clu.registers
        .set_all_flags(&[(clu.registers.a == 0) as u8, 0, 0, 0])?;
    Ok(())
}

//NOTE: Untested
pub fn cp(opcode: u8, clu: &mut CLU) -> Result<(), String> {
    //NOTE: This code is also valid for sub, probably need to do that as well there
    print!("cp ");
    let r8_param = R8::get_r8_param(opcode == 0xFE, opcode, 0, clu)?;
    let subtrahend = match r8_param {
        Register { reg: _, value } => {
            println!("r8");
            value
        }
        R8::Hl { addr: _, value } => {
            println!("[hl]");
            value
        }
        N8(n) => {
            println!("imm8");
            n
        }
    };
    let half_carry = (clu.registers.a & 0xF) < (subtrahend & 0xF);
    let (res, carry) = clu.registers.a.overflowing_sub(subtrahend);
    let zero = res == 0;
    clu.registers
        .set_all_flags(&[zero as u8, 1, half_carry as u8, carry as u8])?;
    Ok(())
}

/// inc r8 | inc hl | dec r8 | dec hl
pub fn inc_r8(opcode: u8, clu: &mut CLU, delta: i8) -> Result<(), String> {
    let r8_param = R8::get_r8_param(false, opcode, 3, clu)?;
    match r8_param {
        Register { reg: _, value } | Hl { addr: _, value } => {
            let (half_carry, zero, sub, res): (bool, bool, bool, u8);
            if delta < 0 {
                print!("dec ");
                res = value.wrapping_sub(delta.unsigned_abs());
                half_carry = (value & 0xF) < (delta.unsigned_abs() & 0xF);
                sub = true
            } else {
                print!("inc ");
                res = value.wrapping_add(delta as u8);
                half_carry = (value & 0xF) + (delta as u8 & 0xF) > 0xF;
                sub = false
            }
            zero = res == 0;
            if let Hl { addr, value: _ } = r8_param {
                clu.clock.tick();
                clu.clock.tick();
                clu.memory.write(addr, res)?;
                println!("[hl]");
            } else if let Register { reg, value: _ } = r8_param {
                *clu.registers.match_r8(reg)? = res;
                println!("r8");
            }
            clu.registers.set_all_flags(&[
                zero as u8,
                sub as u8,
                half_carry as u8,
                clu.registers.read_flag(Flag::Carry) as u8,
            ])?;
        }
        _ => (),
    }
    Ok(())
}
