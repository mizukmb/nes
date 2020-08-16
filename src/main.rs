use std::fs::File;
use std::io::Read;

mod cpu;
mod ines;

fn main() {
    let filename = "./sample1.nes";

    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();

    f.read_to_end(&mut buf).unwrap();

    let ines = ines::new(buf);
    let mut cpu = cpu::new();
    cpu.fetch(&ines.program_rom);
    cpu.exec(&ines.program_rom).unwrap();
    println!("{:?}", cpu);

    // println!("Header: {:?}", ines.header.constant_1); // Header: 78
    // println!("Program ROM Size: {}", ines.header.program_rom_size); // Program ROM Size: 2
    // println!("Program ROM Data Size: {}", ines.program_rom.data.len()); // Program ROM Data Size: 32768
    // println!("Character ROM Size: {}", ines.header.character_rom_size); // Character ROM Size 1
    // println!("Character ROM Data Size: {}", ines.character_rom.data.len()); // Character ROM Data Size: 8192
    // println!("Flag 6: {}", ines.header.flag_6); // Flag 6

    // println!("{:?}", ines.sprite.data);
    // ines.sprite.to_png();
    // println!("Gerenated image.png");
}
