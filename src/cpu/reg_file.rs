use crate::cpu::alu::alu;

// TODO: Use Idiomatic rust names
pub enum Modes {
    DMG,
    MGB,
    CGB,
    CGBDMG,
}

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry,
}

impl Flag {
    fn bit_index(&self) -> u8 {
        match self {
            Flag::Zero => ZERO,
            Flag::Subtract => SUB,
            Flag::HalfCarry => HALFCARRY,
            Flag::Carry => CARRY,
        }
    }
}

const ZERO: u8 = 7;
const SUB: u8 = 6;
const HALFCARRY: u8 = 5;
const CARRY: u8 = 4;

#[derive(Debug)]
pub struct RegFile {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl RegFile {
    pub fn new(mode: Modes) -> Self {
        let (a, b, c, d, e, f, h, l): (u8, u8, u8, u8, u8, u8, u8, u8) = match mode {
            Modes::DMG => (0x1, 0x0, 0x13, 0x00, 0xD8, 0xB0, 0x1, 0x4D),
            Modes::MGB => (0xFF, 0x0, 0x13, 0x00, 0xD8, 0xB0, 0x1, 0x4D),
            Modes::CGB => (0x11, 0x0, 0x0, 0xFF, 0x56, 0x80, 0x0, 0xD),
            Modes::CGBDMG => (0x11, 0x0, 0x0, 0x0, 0x8, 0x80, 0x0, 0x7C),
        };
        RegFile {
            a,
            b,
            c,
            d,
            e,
            f,
            h,
            l,
            sp: 0xFFFE,
            pc: 0x100,
        }
    }

    pub fn match_register(&mut self, num: u8) -> Result<&mut u8, &str> {
        match num {
            0x0 => Ok(&mut self.b),
            0x1 => Ok(&mut self.c),
            0x2 => Ok(&mut self.d),
            0x3 => Ok(&mut self.e),
            0x4 => Ok(&mut self.h),
            0x5 => Ok(&mut self.l),
            0x7 => Ok(&mut self.a),
            _ => Err("Invalid r8 index"),
        }
    }

    // TODO: Refactor to use read_flag
    pub fn match_condition(&self, num: u8) -> Result<bool, &str> {
        match num {
            0x0 => Ok(alu::read_bits(self.f, 4, 1) != 1),
            0x1 => Ok(alu::read_bits(self.f, 4, 1) == 1),
            0x2 => Ok(alu::read_bits(self.f, 7, 1) != 1),
            0x3 => Ok(alu::read_bits(self.f, 7, 1) == 1),
            _ => Err("Invalid condition parameter"),
        }
    }

    pub fn read_flag(&self, flag: Flag) -> bool {
        let index = flag.bit_index();
        let value = alu::read_bits(self.f, index, 1);
        match value {
            1 => true,
            0 => false,
            _ => {
                eprintln!("WARNING: alu::read_bits returned {value} for single bit read");
                true
            }
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: Option<bool>) -> Result<(), String> {
        let index = flag.bit_index();
        match value {
            Some(bit) => alu::write_bits(&mut self.f, index, 1, bit as u8)?,
            None => {
                let bit: bool = self.read_flag(flag);
                alu::write_bits(&mut self.f, index, 1, !bit as u8)?
            }
        }
        Ok(())
    }
}
