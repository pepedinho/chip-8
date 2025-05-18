use super::schema::{Jump, CPU, MEM_SIZE, NBR_OPCODE, START_ADRR};

impl CPU {
    pub fn new() -> Self {
        CPU {
            mem: [0u8; MEM_SIZE],
            V: [0u8; 16],
            jump: [0u16; 16],
            pc: START_ADRR,
            jump_counter: 0,
            game_count: 0,
            sound_count: 0,
            I: 0,
        }
    }

    pub fn countdown(&mut self) {
        if self.game_count > 0 {
            self.game_count -= 1;
        }
        if self.sound_count > 0 {
            self.sound_count -= 1;
        }
    }

    pub fn get_opcode(&mut self) -> u16 {
        return ((self.mem[self.pc as usize] as u16) << 8)
            + self.mem[(self.pc + 1) as usize] as u16;
    }
}

impl Jump {
    pub fn new() -> Self {
        let mut mask = [0u16; NBR_OPCODE];
        let mut id = [0u16; NBR_OPCODE];

        mask[0] = 0x0000;
        id[0] = 0x0FFF; // 0NNN
        mask[1] = 0xFFFF;
        id[1] = 0x00E0; // 00E0
        mask[2] = 0xFFFF;
        id[2] = 0x00EE; // 00EE
        mask[3] = 0xF000;
        id[3] = 0x1000; // 1NNN
        mask[4] = 0xF000;
        id[4] = 0x2000; // 2NNN
        mask[5] = 0xF000;
        id[5] = 0x3000; // 3XNN
        mask[6] = 0xF000;
        id[6] = 0x4000; // 4XNN
        mask[7] = 0xF00F;
        id[7] = 0x5000; // 5XY0
        mask[8] = 0xF000;
        id[8] = 0x6000; // 6XNN
        mask[9] = 0xF000;
        id[9] = 0x7000; // 7XNN
        mask[10] = 0xF00F;
        id[10] = 0x8000; // 8XY0
        mask[11] = 0xF00F;
        id[11] = 0x8001; // 8XY1
        mask[12] = 0xF00F;
        id[12] = 0x8002; // 8XY2
        mask[13] = 0xF00F;
        id[13] = 0x8003; // BXY3
        mask[14] = 0xF00F;
        id[14] = 0x8004; // 8XY4
        mask[15] = 0xF00F;
        id[15] = 0x8005; // 8XY5
        mask[16] = 0xF00F;
        id[16] = 0x8006; // 8XY6
        mask[17] = 0xF00F;
        id[17] = 0x8007; // 8XY7
        mask[18] = 0xF00F;
        id[18] = 0x800E; // 8XYE
        mask[19] = 0xF00F;
        id[19] = 0x9000; // 9XY0
        mask[20] = 0xF000;
        id[20] = 0xA000; // ANNN
        mask[21] = 0xF000;
        id[21] = 0xB000; // BNNN
        mask[22] = 0xF000;
        id[22] = 0xC000; // CXNN
        mask[23] = 0xF000;
        id[23] = 0xD000; // DXYN
        mask[24] = 0xF0FF;
        id[24] = 0xE09E; // EX9E
        mask[25] = 0xF0FF;
        id[25] = 0xE0A1; // EXA1
        mask[26] = 0xF0FF;
        id[26] = 0xF007; // FX07
        mask[27] = 0xF0FF;
        id[27] = 0xF00A; // FX0A
        mask[28] = 0xF0FF;
        id[28] = 0xF015; // FX15
        mask[29] = 0xF0FF;
        id[29] = 0xF018; // FX18
        mask[30] = 0xF0FF;
        id[30] = 0xF01E; // FX1E
        mask[31] = 0xF0FF;
        id[31] = 0xF029; // FX29
        mask[32] = 0xF0FF;
        id[32] = 0xF033; // FX33
        mask[33] = 0xF0FF;
        id[33] = 0xF055; // FX55
        mask[34] = 0xF0FF;
        id[34] = 0xF065; // FX65

        Self { mask, id }
    }

    pub fn get_action(&self, opcode: u16) -> u8 {
        let mut action: u8 = 0;
        while action < NBR_OPCODE as u8 {
            let result = self.mask[action as usize] & opcode;

            if result == self.id[action as usize] {
                break;
            }
            action += 1;
        }
        return action;
    }
}
