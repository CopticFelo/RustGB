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
