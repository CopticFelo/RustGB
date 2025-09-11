use crate::{
    cpu::{
        alu::alu,
        clock::Clock,
        clu::clu::CLU,
        reg_file::{Modes, RegFile},
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
fn ld_r8() -> Result<(), String> {
    let mut clu = get_clu(vec![0x45, 0xDD]);
    let _ = clu.start_exec_cycle();
    assert_eq!(clu.registers.l, clu.registers.b);
    Ok(())
}

#[test]
fn ld_hl() -> Result<(), String> {
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
    Ok(())
}

#[test]
fn ld_to_hl() -> Result<(), String> {
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
    Ok(())
}
