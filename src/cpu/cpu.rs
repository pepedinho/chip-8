use super::schema::{CPU, MEM_SIZE, START_ADRR};

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
}
