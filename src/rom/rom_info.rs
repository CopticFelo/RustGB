pub enum CGBMode {
    Monochrome,
    Color { exclusive: bool },
}

// TODO: Just derive debug lil bro
impl std::fmt::Display for CGBMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            CGBMode::Color { exclusive: true } => "Exclusive",
            CGBMode::Color { exclusive: false } => "Compatible",
            _ => "Monochrome",
        };
        write!(f, "{out}")
    }
}

impl Default for CGBMode {
    fn default() -> Self {
        Self::Color { exclusive: false }
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

impl Default for ROMInfo {
    fn default() -> Self {
        Self {
            title: String::default(),
            cgb: CGBMode::default(),
            sgb: true,
            cartridge_type: 0x10,
            rom_banks: 1,
            mem_banks: 0x3,
            header_checksum: u8::default(),
            rom_checksum: u16::default(),
        }
    }
}
