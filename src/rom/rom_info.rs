pub enum CGBMode {
    Monochrome,
    Color { execlusive: bool },
}

impl std::fmt::Display for CGBMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            CGBMode::Color { execlusive: true } => "Execlusive",
            CGBMode::Color { execlusive: false } => "Compatible",
            _ => "Monochrome",
        };
        write!(f, "{out}")
    }
}

pub struct ROMInfo {
    pub title: String,
    pub cgb: CGBMode,
    pub sgb: bool,
    pub cartridge_type: u8,
    pub rom_banks: u16,
    pub mem_banks: u16,
    pub header_checksum: u8,
    pub rom_checksum: u16,
}
