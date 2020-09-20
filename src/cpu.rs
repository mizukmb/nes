use super::ines::ProgramRom;

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    accumulator: u8,               // アキュムレータ
    index_register_x: u8,          // インデックスレジスタ
    index_register_y: u8,          // インデックスレジスタ
    program_counter: u16,          // プログラムカウンタ
    stack_pointer: u16,            // スタックポインタ
    processor_status_register: u8, // プロセッサステータスレジスタ
    opecode: u8,                   // オペコード
}

pub fn new() -> Cpu {
    Cpu {
        accumulator: 0,
        index_register_x: 0,
        index_register_y: 0,
        program_counter: 0,
        stack_pointer: 0,
        processor_status_register: 0,
        opecode: 0xFF,
    }
}

impl Cpu {
    pub fn fetch(&mut self, rom: &ProgramRom) {
        self.opecode = rom.data[self.program_counter as usize];
        self.step_pc();
    }

    pub fn exec(&mut self, rom: &ProgramRom) -> Result<u8, String> {
        // アドレッシングモード別にはわけてないので後でわける
        match self.opecode.clone() {
            0x00 => Ok(0),
            0x01 => Ok(0),
            0x05 => Ok(0),
            0x06 => Ok(0),
            0x08 => Ok(0),
            0x09 => Ok(0),
            0x0A => Ok(0),
            0x0D => Ok(0),
            0x0E => Ok(0),

            0x10 => Ok(0),
            0x11 => Ok(0),
            0x15 => Ok(0),
            0x16 => Ok(0),
            0x18 => Ok(0),
            0x19 => Ok(0),
            0x1D => Ok(0),
            0x1E => Ok(0),

            0x20 => Ok(0),
            0x21 => Ok(0),
            0x24 => Ok(0),
            0x25 => Ok(0),
            0x26 => Ok(0),
            0x28 => Ok(0),
            0x29 => Ok(0),
            0x2A => Ok(0),
            0x2C => Ok(0),
            0x2D => Ok(0),
            0x2E => Ok(0),

            0x30 => Ok(0),
            0x31 => Ok(0),
            0x35 => Ok(0),
            0x36 => Ok(0),
            0x38 => Ok(0),
            0x39 => Ok(0),
            0x3D => Ok(0),
            0x3E => Ok(0),

            0x40 => Ok(0),
            0x41 => Ok(0),
            0x45 => Ok(0),
            0x46 => Ok(0),
            0x48 => Ok(0),
            0x49 => Ok(0),
            0x4A => Ok(0),
            0x4C => Ok(0),
            0x4D => Ok(0),
            0x4E => Ok(0),

            0x50 => Ok(0),
            0x51 => Ok(0),
            0x55 => Ok(0),
            0x56 => Ok(0),
            0x58 => Ok(0),
            0x59 => Ok(0),
            0x5D => Ok(0),
            0x5E => Ok(0),

            0x60 => Ok(0),
            0x61 => Ok(0),
            0x65 => Ok(0),
            0x66 => Ok(0),
            0x68 => Ok(0),
            0x69 => Ok(0),
            0x6A => Ok(0),
            0x6C => Ok(0),
            0x6D => Ok(0),
            0x6E => Ok(0),

            0x70 => Ok(0),
            0x71 => Ok(0),
            0x75 => Ok(0),
            0x76 => Ok(0),
            0x78 => {
                self.sei();
                Ok(0)
            }
            0x79 => Ok(0),
            0x7D => Ok(0),
            0x7E => Ok(0),

            0x81 => Ok(0),
            0x82 => Ok(0),
            0x84 => Ok(0),
            0x85 => Ok(0),
            0x86 => Ok(0),
            0x88 => Ok(0),
            0x8A => Ok(0),
            0x8C => Ok(0),
            0x8D => Ok(0),
            0x8E => Ok(0),

            0x90 => Ok(0),
            0x91 => Ok(0),
            0x94 => Ok(0),
            0x95 => Ok(0),
            0x96 => Ok(0),
            0x98 => Ok(0),
            0x99 => Ok(0),
            0x9A => Ok(0),
            0x9D => Ok(0),

            0xA0 => Ok(0),
            0xA1 => Ok(0),
            0xA2 => {
                self.ldx(rom.data[self.program_counter as usize + 1]);
                Ok(0)
            }
            0xA4 => Ok(0),
            0xA5 => Ok(0),
            0xA6 => Ok(0),
            0xA8 => Ok(0),
            0xA9 => Ok(0),
            0xAA => Ok(0),
            0xAC => Ok(0),
            0xAD => Ok(0),
            0xAE => Ok(0),

            0xB0 => Ok(0),
            0xB1 => Ok(0),
            0xB4 => Ok(0),
            0xB5 => Ok(0),
            0xB6 => Ok(0),
            0xB8 => Ok(0),
            0xB9 => Ok(0),
            0xBA => Ok(0),
            0xBC => Ok(0),
            0xBD => Ok(0),
            0xBE => Ok(0),

            0xC0 => Ok(0),
            0xC1 => Ok(0),
            0xC4 => Ok(0),
            0xC5 => Ok(0),
            0xC6 => Ok(0),
            0xC8 => Ok(0),
            0xC9 => Ok(0),
            0xCA => Ok(0),
            0xCC => Ok(0),
            0xCD => Ok(0),
            0xCE => Ok(0),

            0xD0 => Ok(0),
            0xD1 => Ok(0),
            0xD5 => Ok(0),
            0xD6 => Ok(0),
            0xD8 => Ok(0),
            0xD9 => Ok(0),
            0xDD => Ok(0),
            0xDE => Ok(0),

            0xE0 => Ok(0),
            0xE1 => Ok(0),
            0xE4 => Ok(0),
            0xE5 => Ok(0),
            0xE6 => Ok(0),
            0xE8 => Ok(0),
            0xE9 => Ok(0),
            0xEA => Ok(0),
            0xEC => Ok(0),
            0xED => Ok(0),
            0xEE => Ok(0),

            0xF0 => Ok(0),
            0xF1 => Ok(0),
            0xF5 => Ok(0),
            0xF6 => Ok(0),
            0xF8 => Ok(0),
            0xF9 => Ok(0),
            0xFD => Ok(0),
            0xFE => Ok(0),

            _ => Err(format!(
                "CANNOT EXEC INSTRUCTION. OPECODE: {}",
                self.opecode
            )),
        }
    }

    fn step_pc(&mut self) {
        self.program_counter += 1;
    }

    fn sei(&mut self) {
        self.processor_status_register = self.processor_status_register | 0b0000_0100;
        println!("instruction SEI");
    }

    fn ldx(&mut self, data: u8) {
        self.index_register_x = data;
        self.program_counter += 1; // dataのロード分1つ多く動かす
        println!("instruction LDX");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_cpu() {
        let expect = new();
        let actual = Cpu {
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            program_counter: 0,
            stack_pointer: 0,
            processor_status_register: 0,
            opecode: 0xFF,
        };

        assert_eq!(expect, actual);
    }

    #[test]
    fn fetch_opecode() {
        let rom = ProgramRom { data: vec![0x78] };
        let mut cpu = new();

        cpu.fetch(&rom);

        let expect = cpu.opecode;
        let actual = 0x78;

        assert_eq!(expect, actual);

        let expect = cpu.program_counter;
        let actual = 1;

        assert_eq!(expect, actual);
    }
}
