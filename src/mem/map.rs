pub struct MemoryMap {
    rom_banks: Vec<Vec<u8>>,
    active_rom_bank: usize,
    vram: Vec<Vec<u8>>,
    active_vram: usize,
    eram: Vec<Vec<u8>>,
    active_eram: usize,
    wram: Vec<Vec<u8>>,
    active_wram: usize,
    oam: Vec<u8>,
    io: Vec<u8>,
    hram: Vec<u8>,
    ie: u8,
}

impl MemoryMap {
    fn read(&self, addr: u16) -> Result<u8, String> {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3FFF => self.rom_banks[0].get(addr),
            0x4000..=0x7FFF => self.rom_banks[self.active_rom_bank].get(addr - 0x4000),
            0x8000..=0x9FFF => self.vram[self.active_vram].get(addr - 0x8000),
            0xA000..=0xBFFF => self.eram[self.active_eram].get(addr - 0xA000),
            0xC000..=0xCFFF => self.wram[0].get(addr - 0xC000),
            0xD000..=0xDFFF => self.wram[self.active_wram].get(addr - 0xD000),
            0xE000..=0xEFFF => self.wram[0].get(addr - 0xE000),
            0xF000..=0xFDFF => self.wram[self.active_wram].get(addr - 0xF000),
            0xFE00..=0xFE9F => self.oam.get(addr - 0xFE00),
            0xFEA0..=0xFEFF => Some(&0),
            0xFF00..=0xFF7F => self.io.get(addr - 0xFF00),
            0xFF80..=0xFFFE => self.hram.get(addr - 0xFF80),
            0xFFFF => Some(&self.ie),
            _ => None,
        }
        .copied()
        .ok_or(format!("Error: Out of bounds address {}", addr))
    }
    fn write(&mut self, addr: u16, value: u8) -> Result<(), String> {
        let addr = addr as usize;
        let opt_mem_ptr: Option<&mut u8> = match addr {
            0x0000..=0x3FFF => {
                return Err(format!("Error: Read-only address {} (ROM bank 0)", addr));
            }
            0x4000..=0x7FFF => {
                return Err(format!(
                    "Error: Read-only address {} (ROM bank {})",
                    addr, self.active_rom_bank
                ));
            }
            0x8000..=0x9FFF => self.vram[self.active_vram].get_mut(addr - 0x8000),
            0xA000..=0xBFFF => self.eram[self.active_eram].get_mut(addr - 0xA000),
            0xC000..=0xCFFF => self.wram[0].get_mut(addr - 0xC000),
            0xD000..=0xDFFF => self.wram[self.active_wram].get_mut(addr - 0xD000),
            0xE000..=0xEFFF => self.wram[0].get_mut(addr - 0xE000),
            0xF000..=0xFDFF => self.wram[self.active_wram].get_mut(addr - 0xF000),
            0xFE00..=0xFE9F => self.oam.get_mut(addr - 0xFE00),
            0xFEA0..=0xFEFF => {
                // https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
                return Err(format!("Error: Invalid address {} (Prohibited)", addr));
            }
            0xFF00..=0xFF7F => self.io.get_mut(addr - 0xFF00),
            0xFF80..=0xFFFE => self.hram.get_mut(addr - 0xFF80),
            0xFFFF => Some(&mut self.ie),
            _ => None,
        };
        if let Some(mem_ptr) = opt_mem_ptr {
            *mem_ptr = value;
            Ok(())
        } else {
            Err(format!("Error: Out of bounds or invalid address {}", addr))
        }
    }
}
