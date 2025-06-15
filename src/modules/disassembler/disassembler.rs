use std::collections::VecDeque;
use crate::modules::disassembler::instructions::{Decoded, INSTRUCTIONS};
pub fn opcode_extract(opcode: u16) ->  Result<Decoded, u16> {
    for instruction in  INSTRUCTIONS.iter() {
        let opcode_demasked = opcode & instruction.mask;
        if opcode_demasked == instruction.pattern {
            // add args extrated from opcode in the decoded
            let mut arguments: Vec<u16> = Vec::new();
            for arg in instruction.arguments {
                let value = (opcode & arg.mask) >> arg.shift;
                arguments.push(value);
            }
            let decoded  = Decoded {
                id: instruction.id.to_string(),
                arguments,
            };
            return Ok(decoded);
        }
    }
    Err(opcode)
}

pub fn decode(result: Result<Decoded, u16>, address: &mut VecDeque<usize>) -> String {
    match result {
        Ok(decoded) => {
            match decoded.id.as_str() {
                "CLEAR_SCREEN" => String::from("CLS"),
                "SUB_RET" => String::from("RET"),
                "JUMP" => format!("JP 0x{:03X}", decoded.arguments[0]),
                "SUB_CALL" => format!("CALL 0x{:03X}", decoded.arguments[0]),
                "IF_VX_EQUALS_LIT" => format!("SE V{:01X}, 0x{:02X}", decoded.arguments[0], decoded.arguments[1]),
                "IF_VX_NOT_EQUALS_LIT" => format!("SNE V{:01X}, 0x{:02X}", decoded.arguments[0], decoded.arguments[1]),
                "IF_VX_EQUALS_VY" => format!("SE V{:01X}, V{:01X}", decoded.arguments[0], decoded.arguments[1]),
                "SET_REG" => format!("LD V{:01X}, 0x{:02X}",  decoded.arguments[0], decoded.arguments[1]),
                "ADD_VALUE" => format!("ADD V{:01X}, 0x{:02X}",  decoded.arguments[0], decoded.arguments[1]),
                "SET_VX_TO_VY" => format!("LD V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "BINARY_OR" => format!("OR V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "BINARY_AND" => format!("AND V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "BINARY_XOR" => format!("XOR V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "ADD_FLAG" => format!("ADD V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "SUB_VY_FROM_VX" => format!("SUB V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "SHIFT_RIGHT" => format!("SHR V{:01X} {{, V{:01X}}}",  decoded.arguments[0], decoded.arguments[1]),
                "SUB_VX_FROM_VY" => format!("SUBN V{:01X}, V{:01X}",  decoded.arguments[0], decoded.arguments[1]),
                "SHIFT_LEFT" => format!("SHL V{:01X} {{, V{:01X}}}",  decoded.arguments[0], decoded.arguments[1]),
                "IF_VX_NOT_EQUALS_VY" => format!("SNE V{:01X}, V{:01X}", decoded.arguments[0], decoded.arguments[1]),
                "SET_INDEX" => {
                    address.push_back(decoded.arguments[0] as usize);
                    format!("LD I, 0x{:03X}", decoded.arguments[0])
                },
                "JUMP_OFFSET" => format!("JP V0, 0x{:03X}", decoded.arguments[0]),
                "RANDOM" => format!("RND V{:01X}, 0x{:02X}", decoded.arguments[0], decoded.arguments[1]),
                "JUMP_KEY_PRESS" => format!("SKP V{:01X}", decoded.arguments[0]),
                "JUMP_KEY_NOT_PRESS" => format!("SKNP V{:01X}", decoded.arguments[0]),
                "SET_DELAY_REG" => format!("LD V{:01X}, DT", decoded.arguments[0]),
                "GET_KEY" => format!("LD V{:01X}, K", decoded.arguments[0]),
                "SET_REG_DELAY" => format!("LD DT, V{:01X}", decoded.arguments[0]),
                "SET_SOUND_TIMER" => format!("LD  ST, V{:01X}", decoded.arguments[0]),
                "ADD_TO_INDEX" => format!("ADD I, V{:01X}", decoded.arguments[0]),
                "SET_INDEX_SPRITE" =>  format!("LD F, V{:01X}", decoded.arguments[0]),
                "BINARY_CODED" =>  format!("LD B, V{:01X}", decoded.arguments[0]),
                "STORE_MEMO" =>  format!("LD [I], V{:01X}", decoded.arguments[0]),
                "LOAD_FROM_MEMO" =>  format!("LD V{:01}, [I]", decoded.arguments[0]),
                "DRAW" => {
                    if decoded.arguments[2] != 0 {
                        if let Some(&start_address) = address.get(address.len() - 1) {
                            let size = decoded.arguments[2] as usize;
                            address.push_back(start_address + size);
                        }
                        format!("DRW V{:01X}, V{:01X}, 0x{:01X}", decoded.arguments[0], decoded.arguments[1], decoded.arguments[2])
                    } else {
                        format!("DRW V{:01}, V{:01}, 0", decoded.arguments[0], decoded.arguments[1])
                    }

                },
                // super
                "SCROLL_DOWN" => format!("SCD 0x{:01} [super set]", decoded.arguments[0]),
                "SCROLL_RIGHT" => String::from("SCR [super set]"),
                "SCROLL_LEFT" => String::from("SCL [super set]"),
                "EXIT" => String::from("EXIT [super set]"),
                "LOW" => String::from("LOW [super set]"),
                "HIGH" => String::from("HIGH [super set]"),
                "INDEX_FONT" => format!("LD HF, V{:01} [super set]", decoded.arguments[0]),
                "STORE_MEMO_SUPER" => format!("LD R, V{:01} [super set]", decoded.arguments[0]),
                "LOAD_FROM_MEMO_SUPER" => format!("LD V{:01}, R [super set]", decoded.arguments[0]),
                _ => String::from("Problem reading line"),
            }
        }
        Err(opcode) => { format!("0x{:X} not founded [problem, data or another chip version]", opcode) }
    }
}