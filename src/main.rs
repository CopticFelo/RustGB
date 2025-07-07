use std::env;
use std::fs;
use std::io;
use std::io::Write;

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

    let rom = fs::read(rom_path).expect("Failed to read ROM file");
    for byte in rom.iter() {
        println!("{:#X?}", byte);
    }
}
