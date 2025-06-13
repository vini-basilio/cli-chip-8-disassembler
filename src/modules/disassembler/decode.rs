use crate::modules::instructions::{Decoded, INSTRUCTIONS};
pub fn opcode_extract(opcode: u16) ->  Result<Decoded, u16> {
    for instruction in  INSTRUCTIONS.iter() {
        let opcode_demasked = opcode & instruction.mask;
        if opcode_demasked == instruction.pattern {

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

pub fn decode(result: Result<Decoded, u16>) -> String {
    match result {
        Ok(decoded) => {
            match decoded.id.as_str() {
                "CLEAR_SCREEN" => format!("CLS"),
                "SUB_RET" => format!("RET"),
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
                "SET_INDEX" => format!("LD I, 0x{:03X}", decoded.arguments[0]),
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
                "DRAW" => format!("DRW V{:01X}, V{:01X}, 0x{:01X}",  decoded.arguments[0], decoded.arguments[1], decoded.arguments[2]),
                _ => format!("OPCODE não implementado!"),
            }
        }
        Err(opcode) => { format!("0x{:3X} not founded", opcode) }
    }
}