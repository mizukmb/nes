extern crate image;

use image::{ImageBuffer, Rgb};

pub struct Ines {
    pub header: Header,
    pub program_rom: ProgramRom,
    pub character_rom: CharacterRom,
    pub sprite: Sprite,
}

pub struct Header {
    pub constant_1: u8,
    pub constant_2: u8,
    pub constant_3: u8,
    pub constant_4: u8,
    pub program_rom_size: u8,
    pub character_rom_size: u8,
    pub flag_6: u8,
    pub flag_7: u8,
    pub flag_8: u8,
    pub flag_9: u8,
    pub flag_10: u8,
}

pub struct ProgramRom {
    pub data: Vec<u8>,
}

pub struct CharacterRom {
    pub data: Vec<u8>,
}

pub struct Sprite {
    pub data: Vec<Vec<Vec<String>>>,
}

pub fn new(nes: Vec<u8>) -> Ines {
    let (header, others) = nes.split_at(16);
    let (program_rom, others) = others.split_at((16384 * header[4] as u64) as usize);
    let (character_rom, _others) = others.split_at((8192 * header[5] as u64) as usize);

    Ines {
        header: new_header(header),
        program_rom: new_program_rom(program_rom.to_vec()),
        character_rom: new_character_rom(character_rom.to_vec()),
        sprite: new_sprite(character_rom.to_vec()),
    }
}

fn new_header(header: &[u8]) -> Header {
    Header {
        constant_1: header[0],
        constant_2: header[1],
        constant_3: header[2],
        constant_4: header[3],
        program_rom_size: header[4],
        character_rom_size: header[5],
        flag_6: header[6],
        flag_7: header[7],
        flag_8: header[8],
        flag_9: header[9],
        flag_10: header[10],
    }
}

fn new_program_rom(program_rom: Vec<u8>) -> ProgramRom {
    ProgramRom { data: program_rom }
}

fn new_character_rom(character_rom: Vec<u8>) -> CharacterRom {
    CharacterRom {
        data: character_rom,
    }
}

fn new_sprite(raw: Vec<u8>) -> Sprite {
    let mut sprites = Vec::new();
    for chank in 0..(raw.len() / 16) {
        let mut sprite = Vec::new();

        for i in 0..8 {
            let mut tmp = Vec::new();
            let (a, b): (Vec<char>, Vec<char>) = (
                format!("{:08b}", raw[i + (chank * 16)]).chars().collect(),
                format!("{:08b}", raw[i + (chank * 16) + 8])
                    .chars()
                    .collect(),
            );

            for pixel in 0..8 {
                tmp.push(format!("{}{}", b[pixel], a[pixel]));
            }

            sprite.push(tmp);
        }

        sprites.push(sprite);
    }

    Sprite { data: sprites }
}

impl Sprite {
    pub fn to_png(&self) {
        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(
            (self.data.len() * 8 + 10) as u32,
            (self.data[0].len()) as u32,
        );
        for chank in 0..(self.data.len()) {
            for y in 0..(self.data[chank].len()) {
                for x in 0..(self.data[chank][y].len()) {
                    println!("{} {} {}", chank, x, y);
                    let rgb = match self.data[chank][y][x].as_str() {
                        "00" => [255, 0, 0],     // [0, 0, 0], // black
                        "01" => [114, 113, 113], // gray (Light)
                        "10" => [204, 204, 204], // gray (Dark)
                        "11" => [255, 255, 255], //white
                        _ => [0, 0, 0],
                    };

                    // set a central pixel to white
                    image.get_pixel_mut((chank * 8 + x) as u32, y as u32).data = rgb;
                }
            }
        }

        // write it out to a file
        image.save("/path/to/image.png").unwrap();
    }
}
