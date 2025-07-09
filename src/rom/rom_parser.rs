use crate::rom::rom_info::{CGBMode, ROMInfo};
use std::fs;
use std::str;

const NINTENDO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

pub fn parse_rom(path: &str) {
    let rom = fs::read(path).unwrap();
    assert!(rom.len() > 0x150, "Invalid ROM File (File too short)");
    assert!(
        validate_rom(&rom),
        "Invalid ROM File (No Nintendo Logo found)"
    );

    let game_title = String::from_utf8_lossy(&rom[0x134..0x143]).to_string();

    let cgb = rom[0x143];
    let cgb_mode = match cgb {
        0x80 => CGBMode::Color { execlusive: false },
        0xC0 => CGBMode::Color { execlusive: true },
        _ => CGBMode::Monochrome,
    };

    let sgb = rom[0x146] == 0x3;

    let cartridge_type = rom[0x147];

    let rom_banks = 2 * (1 << rom[0x148]);

    let mem_banks = match rom[0x149] {
        0x0 => 0,
        0x1 => 1,
        0x3 => 4,
        0x4 => 16,
        0x5 => 8,
        _ => 0,
    };

    let header_checksum = rom[0x14D];
    assert!(
        validate_header_checksum(&rom[0x134..=0x14C], header_checksum),
        "Invalid Header checksum"
    );

    let rom_checksum = {
        let bytes = &rom[0x14E..=0x14F];
        ((bytes[0] as u16) << 8) | bytes[1] as u16
    };

    let info = ROMInfo {
        title: game_title,
        cgb: cgb_mode,
        sgb,
        cartridge_type,
        rom_banks,
        mem_banks,
        header_checksum,
        rom_checksum,
    };
    println!("{}", info.title);
    println!("{}", info.cgb);
}

fn validate_header_checksum(header: &[u8], checksum: u8) -> bool {
    let mut calculated_checksum: u8 = 0;
    for byte in header.iter() {
        calculated_checksum = calculated_checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    println!("Calculated checksum: {calculated_checksum:#x}");
    println!("Header checksum: {checksum:#x}");
    calculated_checksum == checksum
}

fn validate_rom(rom: &[u8]) -> bool {
    rom[0x104..=0x133] == NINTENDO
}
