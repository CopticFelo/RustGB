use std::io::Write;
use std::{env, fs, io};

mod rom;
use rom::rom_info;
use rom::rom_parser;

mod cpu;
mod emulator;
mod mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rom_path: String = String::new();
    #[cfg(not(debug_assertions))]
    {
        if args.len() < 2 {
            print!("Input ROM:");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut rom_path)
                .expect("Input error occoured");
            rom_path = rom_path.trim().to_string();
        } else {
            rom_path = args[1].clone();
        }
    }
    #[cfg(debug_assertions)]
    {
        rom_path = "/home/felo/dev/rust/RustGB/test_roms/tetris.gb".to_string();
    }
    println!("Reading input rom: {rom_path}");
    let rom = fs::read(rom_path).expect("Failed to read file");
    let info: rom_info::ROMInfo = rom_parser::parse_rom_header(&rom);
    match emulator::init_emulation(rom, info) {
        Ok(()) => (),
        Err(s) => eprintln!("{}", s),
    }
}
