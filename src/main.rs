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
    if args.len() < 2 {
        print!("Input ROM:");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut rom_path)
            .expect("Input error occoured");
    } else {
        rom_path = args[1].clone();
    }
    println!("Reading input rom: {rom_path}");
    let rom = fs::read(rom_path).expect("Failed to read file");
    let info: rom_info::ROMInfo = rom_parser::parse_rom_header(&rom);
    emulator::init_emulation(rom, info);
}
