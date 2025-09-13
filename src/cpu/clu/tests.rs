use crate::{
    cpu::{
        alu::alu,
        clock::Clock,
        clu::clu::CLU,
        reg_file::{Flag, Modes, RegFile},
    },
    mem::map::MemoryMap,
    rom::rom_info::ROMInfo,
};

fn get_clu(rom: Vec<u8>) -> CLU {
    let mut clu = CLU::init(
        RegFile::new(Modes::CGBDMG),
        MemoryMap::init_rom(rom, ROMInfo::default()),
        Clock::default(),
    );
    clu.registers.pc = 0;
    clu
}

#[test]
fn ld_b_l() -> Result<(), String> {
    let mut clu = get_clu(vec![0x45, 0xDD]);
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.l, clu.registers.b);
    assert_eq!(clu.clock.m_cycles, 2);
    Ok(())
}

#[test]
fn ld_b_hl() -> Result<(), String> {
    let mut clu = get_clu(vec![0x46, 0xDD]);
    let _ = clu.memory.write(0xC001, 0xB1);
    alu::write_u16(&mut clu.registers.l, &mut clu.registers.h, 0xC001);
    let _ = clu.start_exec_cycle();
    assert_eq!(
        clu.memory
            .read(alu::read_u16(&clu.registers.l, &clu.registers.h))
            .unwrap(),
        clu.registers.b
    );
    assert_eq!(clu.clock.m_cycles, 3);
    Ok(())
}

#[test]
fn ld_hl_a() -> Result<(), String> {
    let mut clu = get_clu(vec![0x77, 0xDD]);
    clu.registers.a = 0xB1;
    alu::write_u16(&mut clu.registers.l, &mut clu.registers.h, 0xC001);
    let _ = clu.start_exec_cycle();
    assert_eq!(
        clu.memory
            .read(alu::read_u16(&clu.registers.l, &clu.registers.h))
            .unwrap(),
        clu.registers.a
    );
    assert_eq!(clu.clock.m_cycles, 3);
    Ok(())
}

#[test]
fn jmp() -> Result<(), String> {
    let mut clu = get_clu(vec![0xC3, 0x4, 0x0, 0xDD, 0xDD]);
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.pc, 5);
    assert_eq!(clu.clock.m_cycles, 5);
    Ok(())
}

#[test]
fn jmp_nz() -> Result<(), String> {
    // Jumps
    let mut clu = get_clu(vec![0xC2, 0x4, 0x0, 0xDD, 0xDD]);
    let _ = clu.registers.set_flag(Flag::Zero, Some(false));
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.pc, 5);
    assert_eq!(clu.clock.m_cycles, 5);

    // Doesn't jump
    clu.registers.pc = 0;
    clu.clock.m_cycles = 0;
    clu.clock.t_cycles = 0;
    let _ = clu.registers.set_flag(Flag::Zero, Some(true));
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.pc, 4);
    assert_eq!(clu.clock.m_cycles, 4);
    Ok(())
}
