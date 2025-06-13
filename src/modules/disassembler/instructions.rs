
#[derive(Debug, Clone)]
pub struct Argument {
    pub mask: u16,
    pub shift: u8,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub id: &'static str,
    pub mask: u16,
    pub pattern: u16,
    pub arguments: &'static [Argument],
}

pub struct Decoded {
    pub id: String,
    pub arguments: Vec<u16>
}


macro_rules! instruction {
    ($id:expr, $mask:expr, $pattern:expr, [$($arg_mask:expr, $arg_shift:expr),*]) => {
        Instruction {
            id: $id,
            mask: $mask,
            pattern: $pattern,
            arguments: &[$(Argument { mask: $arg_mask, shift: $arg_shift }),*],
        }
    };
}
pub const INSTRUCTIONS: &[Instruction] = &[
    instruction!("CLEAR_SCREEN", 0xFFFF, 0x00E0, []),
    instruction!("SUB_RET", 0xFFFF, 0x00EE, []),
    instruction!("SUB_CALL", 0xF000, 0x2000, [0x0FFF, 0]),
    instruction!("JUMP", 0xF000, 0x1000, [0x0FFF, 0]),
    instruction!("IF_VX_EQUALS_LIT", 0xF000, 0x3000, [0x0F00, 8, 0x00FF, 0]),
    instruction!("IF_VX_NOT_EQUALS_LIT", 0xF000, 0x4000, [0x0F00, 8, 0x00FF, 0]),
    instruction!("IF_VX_EQUALS_VY", 0xF000, 0x5000, [0x0F00, 8, 0x00F0, 4]),
    instruction!("SET_REG", 0xF000, 0x6000, [0x0F00, 8, 0x00FF, 0]),
    instruction!("ADD_VALUE", 0xF000, 0x7000, [0x0F00, 8, 0x00FF, 0]),
    instruction!("SET_VX_TO_VY", 0xF00F, 0x8000, [0x0F00, 8, 0x00F0, 4]),
    instruction!("BINARY_OR", 0xF00F, 0x8001, [0x0F00, 8, 0x00F0, 4]),
    instruction!("BINARY_AND", 0xF00F, 0x8002, [0x0F00, 8, 0x00F0, 4]),
    instruction!("BINARY_XOR", 0xF00F, 0x8003, [0x0F00, 8, 0x00F0, 4]),
    instruction!("ADD_FLAG", 0xF00F, 0x8004, [0x0F00, 8, 0x00F0, 4]),
    instruction!("SUB_VY_FROM_VX", 0xF00F, 0x8005, [0x0F00, 8, 0x00F0, 4]),
    instruction!("SHIFT_RIGHT", 0xF00F, 0x8006, [0x0F00, 8, 0x00F0, 4]),
    instruction!("SUB_VX_FROM_VY", 0xF00F, 0x8007, [0x0F00, 8, 0x00F0, 4]),
    instruction!("SHIFT_LEFT", 0xF00F, 0x800E, [0x0F00, 8, 0x00F0, 4]),
    instruction!("IF_VX_EQUALS_VY", 0xF000, 0x9000, [0x0F00, 8, 0x00F0, 4]),
    instruction!("SET_INDEX", 0xF000, 0xA000, [0x0FFF, 0]),
    instruction!("JUMP_OFFSET", 0xF000, 0xB000, [0x0FFF, 0]),
    instruction!("RANDOM", 0xF000, 0xC000, [0x0F00, 8, 0x00FF, 0]),
    instruction!("DRAW", 0xF000, 0xD000, [0x0F00, 8, 0x00F0, 4, 0x000F, 0]),
    instruction!("JUMP_KEY_PRESS", 0xF0FF, 0xE09E, [0x0F00, 8]),
    instruction!("JUMP_KEY_NOT_PRESS", 0xF0FF, 0xE0A1, [0x0F00, 8]),
    instruction!("SET_DELAY_REG", 0xF0FF, 0xF007, [0x0F00, 8]),
    instruction!("GET_KEY", 0xF0FF, 0xF00A, [0x0F00, 8]),
    instruction!("SET_REG_DELAY", 0xF0FF, 0xF015, [0x0F00, 8]),
    instruction!("SET_SOUND_TIMER", 0xF0FF, 0xF018, [0x0F00, 8]),
    instruction!("ADD_TO_INDEX", 0xF0FF, 0xF01E, [0x0F00, 8]),
    instruction!("SET_INDEX_SPRITE", 0xF0FF, 0xF029, [0x0F00, 8]),
    instruction!("BINARY_CODED", 0xF0FF, 0xF033, [0x0F00, 8]),
    instruction!("STORE_MEMO", 0xF0FF, 0xF055, [0x0F00, 8]),
    instruction!("LOAD_FROM_MEMO", 0xF0FF, 0xF065, [0x0F00, 8]),
    // super chip-8
];
