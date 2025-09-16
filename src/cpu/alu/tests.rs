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
fn add_a_b() -> Result<(), String> {
    let mut clu = get_clu(vec![0x80, 0xDD]);
    clu.registers.a = 172;
    clu.registers.b = 108;
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.a, 24);
    assert_eq!(clu.clock.m_cycles, 2);
    assert!(clu.registers.read_flag(Flag::Carry));
    assert!(clu.registers.read_flag(Flag::HalfCarry));
    assert!(!clu.registers.read_flag(Flag::Zero));
    assert!(!clu.registers.read_flag(Flag::Subtract));

    clu.registers.a = 0;
    clu.registers.b = 0;
    clu.registers.pc = 0;
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.clock.m_cycles, 4);
    assert!(!clu.registers.read_flag(Flag::Carry));
    assert!(!clu.registers.read_flag(Flag::HalfCarry));
    assert!(clu.registers.read_flag(Flag::Zero));
    assert!(!clu.registers.read_flag(Flag::Subtract));
    Ok(())
}

#[test]
fn adc_a_hl() -> Result<(), String> {
    let mut clu = get_clu(vec![0x8E, 0xDD]);
    clu.registers.a = 172;
    alu::write_u16(&mut clu.registers.l, &mut clu.registers.h, 0xC001);
    let _ = clu.memory.write(0xC001, 108);
    let _ = clu.registers.set_flag(Flag::Carry, Some(true));
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.a, 25);
    assert_eq!(clu.clock.m_cycles, 3);
    assert!(clu.registers.read_flag(Flag::Carry));
    assert!(clu.registers.read_flag(Flag::HalfCarry));
    assert!(!clu.registers.read_flag(Flag::Zero));
    assert!(!clu.registers.read_flag(Flag::Subtract));

    clu.registers.a = 255;
    let _ = clu.memory.write(0xC001, 0);
    clu.registers.pc = 0;
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.clock.m_cycles, 6);
    assert!(clu.registers.read_flag(Flag::Carry));
    assert!(clu.registers.read_flag(Flag::HalfCarry));
    assert!(clu.registers.read_flag(Flag::Zero));
    assert!(!clu.registers.read_flag(Flag::Subtract));
    Ok(())
}

#[test]
fn sub_a_c() -> Result<(), String> {
    let mut clu = get_clu(vec![0x91, 0xDD]);
    clu.registers.a = 64;
    clu.registers.c = 108;
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.a, 212);
    assert_eq!(clu.clock.m_cycles, 2);
    assert!(clu.registers.read_flag(Flag::Carry));
    assert!(clu.registers.read_flag(Flag::HalfCarry));
    assert!(!clu.registers.read_flag(Flag::Zero));
    assert!(clu.registers.read_flag(Flag::Subtract));

    clu.registers.a = 0;
    clu.registers.c = 0;
    clu.registers.pc = 0;
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.clock.m_cycles, 4);
    assert!(!clu.registers.read_flag(Flag::Carry));
    assert!(!clu.registers.read_flag(Flag::HalfCarry));
    assert!(clu.registers.read_flag(Flag::Zero));
    assert!(clu.registers.read_flag(Flag::Subtract));
    Ok(())
}

#[test]
fn sbc_a_hl() -> Result<(), String> {
    let mut clu = get_clu(vec![0x9E, 0xDD]);
    clu.registers.a = 64;
    alu::write_u16(&mut clu.registers.l, &mut clu.registers.h, 0xC001);
    let _ = clu.memory.write(0xC001, 108);
    let _ = clu.registers.set_flag(Flag::Carry, Some(true));
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.a, 211);
    assert_eq!(clu.clock.m_cycles, 3);
    assert!(clu.registers.read_flag(Flag::Carry));
    assert!(clu.registers.read_flag(Flag::HalfCarry));
    assert!(!clu.registers.read_flag(Flag::Zero));
    assert!(clu.registers.read_flag(Flag::Subtract));

    clu.registers.a = 1;
    let _ = clu.memory.write(0xC001, 0);
    clu.registers.pc = 0;
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.clock.m_cycles, 6);
    assert!(!clu.registers.read_flag(Flag::Carry));
    assert!(!clu.registers.read_flag(Flag::HalfCarry));
    assert!(clu.registers.read_flag(Flag::Zero));
    assert!(clu.registers.read_flag(Flag::Subtract));
    Ok(())
}
