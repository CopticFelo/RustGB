#[derive(Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

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
    fn default() -> CPU {
        CPU {
            a: 0,
            b: 0,
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
    pub fn read_reg(r: Register) -> u8 {
        // TODO: Implement read_reg
        // read register value (u8 or u16) based on r
    }
    pub fn set_reg(r: Register, value: u16) -> bool {
        // TODO: Implement set_reg
        // Set register value (u8 or u16) based on r
    }
}
