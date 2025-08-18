pub fn read_u16(lo: &u8, hi: &u8) -> u16 {
    (*hi as u16) << 8 | *lo as u16
}
pub fn write_u16(lo: &mut u8, hi: &mut u8, value: u16) {
    *hi = (value >> 8) as u8;
    *lo = value as u8;
}
