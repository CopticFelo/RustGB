use std::env;
use std::io;
use std::io::Write;

mod rom;
use rom::rom_info;
use rom::rom_parser;

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
    let info: rom_info::ROMInfo = rom_parser::parse_rom(&rom_path);
}
