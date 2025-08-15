#[derive(Debug)]
struct RegFile {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pub sp: u16,
    pc: u16,
}

impl Default for RegFile {
    fn default() -> RegFile {
        RegFile {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE,
            pc: 0x100,
        }
    }
}

impl RegFile {
    pub fn read_u16(hi: &u8, lo: &u8) -> u16 {
        (*hi as u16) << 8 | *lo as u16
    }
    pub fn write_u16(hi: &mut u8, lo: &mut u8, value: u16) {
        *hi = (value >> 8) as u8;
        *lo = value as u8;
    }
}
