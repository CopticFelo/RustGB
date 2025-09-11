use crate::cpu::clock::Clock;
use crate::cpu::clu::clu::CLU;
use crate::cpu::reg_file::{Modes, RegFile};
use crate::mem::map;
use crate::rom::rom_info::ROMInfo;

pub fn init_emulation(rom: Vec<u8>, header_data: ROMInfo) -> Result<(), String> {
    let registers = RegFile::new(Modes::DMG);
    let memory = map::MemoryMap::init_rom(rom, header_data);
    let clock = Clock::default();
    let mut clu = CLU::init(registers, memory, clock);
    clu.start_exec_cycle()?;
    Ok(())
}
