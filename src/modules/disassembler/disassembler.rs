use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::modules::decode::{opcode_extract, decode};

pub struct Disassembler {
}
impl Disassembler {
    pub fn disassemble(rom: &str) -> Result<(), Box<dyn Error>>{
        let reader = BufReader::new(File::open(rom).unwrap());
        let mut buffer = Vec::new();

        for byte_or_error in reader.bytes() {
            let byte = byte_or_error.unwrap();
            buffer.push(byte);
        }

        let mut index = 0usize;
        let mut result: Vec<String> = Vec::new();
        let mut pc = 0x200;
        while index < buffer.len(){
            let high = buffer[index] as u16;
            index += 1;
            let low = buffer[index] as u16;
            index += 1;
            let opcode = (high << 8) | low;
            let assembly = opcode_extract(opcode);
            result.push(format!("0x{:03X}: {}", pc, decode(assembly)));
            pc += 2;
        };

        Ok(())
    }
}
