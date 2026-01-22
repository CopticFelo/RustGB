use redgb::{
    cpu::{
        clock::Clock,
        cpu_context::CpuContext,
        reg_file::{Flag, Modes, RegFile},
    },
    mem::map::MemoryMap,
    rom::rom_info::ROMInfo,
};

fn get_mock_context(rom: Vec<u8>) -> CpuContext {
    let mut context = CpuContext::init(
        RegFile::new(Modes::CGBDMG),
        MemoryMap::init_rom(rom, ROMInfo::default()),
        Clock::default(),
    );
    context.registers.pc = 0;
    context
}

#[test]
fn jmp() -> Result<(), String> {
    let mut context = get_mock_context(vec![0xC3, 0x4, 0x0, 0xDD, 0xDD]);
    let _ = context.start_exec_cycle();
    assert_eq!(context.registers.pc, 5);
    assert_eq!(context.clock.m_cycles, 5);
    Ok(())
}

#[test]
fn jmp_nz() -> Result<(), String> {
    // Jumps
    let mut context = get_mock_context(vec![0xC2, 0x4, 0x0, 0xDD, 0xDD]);
    let _ = context.registers.set_flag(Flag::Zero, Some(false));
    let _ = context.start_exec_cycle();
    assert_eq!(context.registers.pc, 5);
    assert_eq!(context.clock.m_cycles, 5);

    // Doesn't jump
    context.registers.pc = 0;
    context.clock.m_cycles = 0;
    context.clock.t_cycles = 0;
    let _ = context.registers.set_flag(Flag::Zero, Some(true));
    let _ = context.start_exec_cycle();
    assert_eq!(context.registers.pc, 4);
    assert_eq!(context.clock.m_cycles, 4);
    Ok(())
}
