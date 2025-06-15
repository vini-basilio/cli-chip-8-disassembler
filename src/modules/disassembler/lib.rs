use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::modules::disassembler::disassembler::{decode, opcode_extract};

pub struct disassembler {
    pc: usize,
    rom_start_at: usize,
}
impl disassembler {
    pub fn new(rom_start_at: usize) -> Self{
        Self {
            pc:rom_start_at,
            rom_start_at,
        }
    }
    pub fn run(&mut self, rom: &str) -> Result<Vec<String>, Box<dyn Error>>{
        let reader = BufReader::new(File::open(rom).unwrap());
        let mut buffer = Vec::new();

        for byte_or_error in reader.bytes() {
            let byte = byte_or_error.unwrap();
            buffer.push(byte);
        }

        let mut index = 0usize;
        let mut result: Vec<String> = Vec::new();
        let mut address_data: VecDeque<usize> = VecDeque::new();
        
        while index < buffer.len() - 2 {
           if address_data.len() > 1 {
                let start_address = address_data[0].saturating_sub(self.rom_start_at);
                if index == start_address {
                    let final_address = address_data[1].saturating_sub(self.rom_start_at);
                    while index < final_address  {
                        result.push(format!("0x{:03X}: 0x{:02X} [asset]", self.pc, buffer[index]));
                        self.pc += 1;
                        index += 1;
                    }
                    address_data.pop_front();
                    address_data.pop_front();
                    continue;
                }
            }
            
            let high = buffer[index] as u16;
            index += 1;
            
            let low = buffer[index] as u16;
            index += 1;
            
            let opcode = (high << 8) | low;
            let assembly = opcode_extract(opcode);
            result.push(format!("0x{:03X}: {}", self.pc, decode(assembly, &mut address_data)));
            self.pc += 2;
        }
        Ok(result)
    }
}
