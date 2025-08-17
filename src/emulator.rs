use crate::cpu::clock::Clock;
use crate::cpu::clu::CLU;
use crate::cpu::reg_file::{Modes, RegFile};
use crate::mem::map;
use crate::rom::rom_info::ROMInfo;

pub fn init_emulation(rom: Vec<u8>, header_data: ROMInfo) {
    let mut registers = RegFile::new(Modes::DMG);
    let mut rom_banks: Vec<Vec<u8>> = Vec::new();
    for bank in rom.chunks(0x4000) {
        rom_banks.push(bank.to_vec());
    }
    drop(rom);
    // TODO: Make constructor for MemoryMap instead of making all fields pub
    let memory = map::MemoryMap {
        active_rom_bank: 1,
        rom_banks,
        vram: vec![vec![0; 0x2000]; 2],
        active_vram: 0,
        eram: vec![vec![0; 0x2000]; header_data.mem_banks as usize],
        active_eram: 1,
        wram: vec![vec![0; 0x2000]; 8],
        active_wram: 1,
        oam: vec![0; 0x100],
        io: vec![0; 0x80],
        hram: vec![0; 0x7E],
        ie: 0,
    };
    let mut clock = Clock::default();
    let mut clu = CLU::init(&mut registers, &memory, &mut clock);
    clu.start_exec_cycle();
}
