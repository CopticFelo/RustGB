pub enum Modes {
    DMG,
    MGB,
    CGB,
    CGBDMG,
}

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
}
