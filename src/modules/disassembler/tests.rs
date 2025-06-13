#[cfg(test)]
mod tests {
    use crate::modules::disassembler::decode::{decode, opcode_extract};

    #[test]
    fn cls(){
        // Teste do opcode: 0x00E0
        let assembly = opcode_extract(0x00E0);
        let result = decode(assembly);
        assert_eq!(result, "CLS");
    }
    #[test]
    fn ret(){
        // Teste do opcode: 0x00E0
        let assembly = opcode_extract(0x00EE);
        let result = decode(assembly);
        assert_eq!(result, "RET");
    }
    #[test]
    fn jp(){
        // Teste do opcode: 0x1NNN
        let assembly = opcode_extract(0x1300);
        let result = decode(assembly);
        assert_eq!(result, "JP 0x300");
    }
    #[test]
    fn call(){
        // Teste do opcode: 0x2NNN
        let assembly = opcode_extract(0x2200);
        let result = decode(assembly);
        assert_eq!(result, "CALL 0x200");
    }
    #[test]
    fn se(){
        // Teste do opcode: 0x3NNN
        let assembly = opcode_extract(0x3A01);
        let result = decode(assembly);
        assert_eq!(result, "SE VA, 0x01");
    }
    #[test]
    fn sne(){
        // Teste do opcode: 0x4NNN
        let assembly = opcode_extract(0x4A01);
        let result = decode(assembly);
        assert_eq!(result, "SNE VA, 0x01");
    }
    #[test]
    fn se_regs(){
        // Teste do opcode: 0x5NN0
        let assembly = opcode_extract(0x5AB0);
        let result = decode(assembly);
        assert_eq!(result, "SE VA, VB");
    }
    #[test]
    fn ld_reg(){
        // Teste do opcode: 0x6XNN
        let assembly = opcode_extract(0x6AB4);
        let result = decode(assembly);
        assert_eq!(result, "LD VA, 0xB4");
    }
    #[test]
    fn add_no_flags(){
        // Teste do opcode: 0x7XNN
        let assembly = opcode_extract(0x7C08);
        let result = decode(assembly);
        assert_eq!(result, "ADD VC, 0x08");
    }
    #[test]
    fn or(){
        // Teste do opcode: 0x8XY1
        let assembly = opcode_extract(0x8AB1);
        let result = decode(assembly);
        assert_eq!(result, "OR VA, VB");
    }
    #[test]
    fn and(){
        // Teste do opcode: 0x8XY2
        let assembly = opcode_extract(0x8AB2);
        let result = decode(assembly);
        assert_eq!(result, "AND VA, VB");
    }
    #[test]
    fn xor(){
        // Teste do opcode: 0x8XY3
        let assembly = opcode_extract(0x8AB3);
        let result = decode(assembly);
        assert_eq!(result, "XOR VA, VB");
    }
    #[test]
    fn add(){
        // Teste do opcode: 0x8XY4
        let assembly = opcode_extract(0x8AB4);
        let result = decode(assembly);
        assert_eq!(result, "ADD VA, VB");
    }
    #[test]
    fn sub(){
        // Teste do opcode: 0x8XY5
        let assembly = opcode_extract(0x8AB5);
        let result = decode(assembly);
        assert_eq!(result, "SUB VA, VB");
    }
    #[test]
    fn shr(){
        // Teste do opcode: 0x8XY6
        let assembly = opcode_extract(0x8AB6);
        let result = decode(assembly);
        assert_eq!(result, "SHR VA {, VB}");
    }
    #[test]
    fn subn(){
        // Teste do opcode: 0x8XY7
        let assembly = opcode_extract(0x8AB7);
        let result = decode(assembly);
        assert_eq!(result, "SUBN VA, VB");
    }
    #[test]
    fn shl(){
        // Teste do opcode: 0x8XYE
        let assembly = opcode_extract(0x8ABE);
        let result = decode(assembly);
        assert_eq!(result, "SHL VA {, VB}");
    }

    #[test]
    fn jp_offset() {
        // Teste do opcode: 0xANNN
        let assembly = opcode_extract(0xA001);
        let result = decode(assembly);
        assert_eq!(result, "LD I, 0x001");
    }
    #[test]
    fn rnd(){
        // Teste do opcode: 0xCXNN
        let assembly = opcode_extract(0xCA01);
        let result = decode(assembly);
        assert_eq!(result, "RND VA, 0x01");
    }
    #[test]
    fn drw(){
        // Teste do opcode: 0xDXYN
        let assembly = opcode_extract(0xDAB1);
        let result = decode(assembly);
        assert_eq!(result, "DRW VA, VB, 0x1");
    }
    #[test]
    fn jp_k_press(){
        // Teste do opcode: 0xEX9E
        let assembly = opcode_extract(0xE09E);
        let result = decode(assembly);
        assert_eq!(result, "SKP V0");
    }
    #[test]
    fn jp_not_k_press(){
        // Teste do opcode: 0xEXA1
        let assembly = opcode_extract(0xE0A1);
        let result = decode(assembly);
        assert_eq!(result, "SKNP V0");
    }
    #[test]
    fn ld_delay(){
        // Teste do opcode: 0xFX07
        let assembly = opcode_extract(0xFF07);
        let result = decode(assembly);
        assert_eq!(result, "LD VF, DT");
    }
    #[test]
    fn ld_k(){
        // Teste do opcode: 0xFX0A
        let assembly = opcode_extract(0xFB0A);
        let result = decode(assembly);
        assert_eq!(result, "LD VB, K");
    }
    #[test]
    fn ld_dt_vx(){
        // Teste do opcode: 0xFX15
        let assembly = opcode_extract(0xF015);
        let result = decode(assembly);
        assert_eq!(result, "LD DT, V0");
    }
    #[test]
    fn add_i_vx(){
        // Teste do opcode: 0xFX1E
        let assembly = opcode_extract(0xF01E);
        let result = decode(assembly);
        assert_eq!(result, "ADD I, V0");
    }
    #[test]
    fn ld_f_vx(){
        // Teste do opcode: 0xFX29
        let assembly = opcode_extract(0xF029);
        let result = decode(assembly);
        assert_eq!(result, "LD F, V0");
    }
    #[test]
    fn ld_b_vx(){
        // Teste do opcode: 0xFX33
        let assembly = opcode_extract(0xF033);
        let result = decode(assembly);
        assert_eq!(result, "LD B, V0");
    }
    #[test]
    fn store_memo(){
        // Teste do opcode: 0xFX55
        let assembly = opcode_extract(0xF055);
        let result = decode(assembly);
        assert_eq!(result, "LD [I], V0");
    }
    #[test]
    fn load_memo(){
        // Teste do opcode: 0xFX65
        let assembly = opcode_extract(0xF065);
        let result = decode(assembly);
        assert_eq!(result, "LD V0, [I]");
    }
}