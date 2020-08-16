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
            0x78 => {
                self.sei();
                Ok(0)
            }
            0xA2 => {
                self.ldx(rom.data[self.program_counter as usize + 1]);
                Ok(0)
            }
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
