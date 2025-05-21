use crate::display::schema::ContextPixels;

use super::schema::{Jump, CPU, MEM_SIZE, NBR_OPCODE, START_ADRR};

impl CPU {
    pub fn new() -> Self {
        CPU {
            mem: [0u8; MEM_SIZE],
            V: [0u8; 16],
            stack: [0u16; 16],
            pc: START_ADRR,
            sp: 0,
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

    pub fn interpreter(&mut self, opcode: u16, j: Jump, display: &mut ContextPixels) {
        // recuperation des sous partie de lopcode
        let b3 = (opcode & (0x0F00)) >> 8; // on prend les 4 bits, b3 représente X
        let b2 = (opcode & (0x00F0)) >> 4; // idem, b2 représente Y
        let b1 = opcode & (0x000F); // on prend les 4 bits de poids faible

        let b4 = j.get_action(opcode);

        match b4 {
            0 => {}                      // opcode non implementer
            1 => display.clear_screen(), // efface l'ecran
            2 => {
                // 00EE revien du saut
                if self.sp == 0 {
                    panic!("Stack underflow: retour sans appel de sous-programme")
                }

                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            3 => {
                let nnn = opcode & 0x0FFF; // les 12 dernier bits | 4 premier = [numeros d'instruction (2)] | 12 dernier = [adrr]
                self.pc = nnn;
                // 1NNN effectue un saut à l'adresse 1NNN.
            }
            4 => {
                // 2NNN appelle le sous-programme en NNN, mais on revient ensuite.
            }
            5 => {
                // 3XNN saute l'instruction suivante si VX est égal à NN.
            }
            6 => {
                // 4XNN saute l'instruction suivante si VX et NN ne sont pas égaux.
            }
            7 => {
                // 5XY0 saute l'instruction suivante si VX et VY sont égaux.
            }
            8 => {
                // 6XNN définit VX à NN.
            }
            9 => {
                // 7XNN ajoute NN à VX.
            }
            10 => {
                // 8XY0 définit VX à la valeur de VY.
            }
            11 => {
                // 8XY1 définit VX à VX OR VY.
            }
            12 => {
                // 8XY2 définit VX à VX AND VY.
            }
            13 => {
                // 8XY3 définit VX à VX XOR VY.
            }
            14 => {
                // 8XY4 ajoute VY à VX. VF est mis à 1 quand il y a un dépassement de mémoire (carry), et à 0 quand il n'y en pas.
            }
            15 => {
                // 8XY5 VY est soustraite de VX. VF est mis à 0 quand il y a un emprunt, et à 1 quand il n'y a en pas.
            }
            16 => {
                // 8XY6 décale (shift) VX à droite de 1 bit. VF est fixé à la valeur du bit de poids faible de VX avant le décalage.
            }
            17 => {
                // 8XY7 VX = VY - VX. VF est mis à 0 quand il y a un emprunt et à 1 quand il n'y en a pas.
            }
            18 => {
                // 8XYE décale (shift) VX à gauche de 1 bit. VF est fixé à la valeur du bit de poids fort de VX avant le décalage.
            }
            19 => {
                // 9XY0 saute l'instruction suivante si VX et VY ne sont pas égaux.
            }
            20 => {
                // ANNN affecte NNN à I.
            }
            21 => {
                // BNNN passe à l'adresse NNN + V0.
            }
            22 => {
                // CXNN définit VX à un nombre aléatoire inférieur à NN.
            }
            23 => {
                // DXYN dessine un sprite aux coordonnées (VX, VY).
                // display.draw_pixel(b1, b2, b3);
            }
            24 => {
                // EX9E saute l'instruction suivante si la clé stockée dans VX est pressée.
            }
            25 => {
                // EXA1 saute l'instruction suivante si la clé stockée dans VX n'est pas pressée.
            }
            26 => {
                // FX07 définit VX à la valeur de la temporisation.
            }
            27 => {
                // FX0A attend l'appui sur une touche et la stocke ensuite dans VX.
            }
            28 => {
                // FX15 définit la temporisation à VX.
            }
            29 => {
                // FX18 définit la minuterie sonore à VX.
            }
            30 => {
                // FX1E ajoute à VX I. VF est mis à 1 quand il y a overflow (I+VX>0xFFF), et à 0 si tel n'est pas le cas.
            }
            31 => {
                // FX29 définit I à l'emplacement du caractère stocké dans VX. Les caractères 0-F (en hexadécimal) sont représentés par une police 4x5.
            }
            32 => {
                // FX33 stocke dans la mémoire le code décimal représentant VX (dans I, I+1, I+2).
            }
            33 => {
                // FX55 stocke V0 à VX en mémoire à partir de l'adresse I.
            }
            34 => {
                // FX65 remplit V0 à VX avec les valeurs de la mémoire à partir de l'adresse I.
            }
            _ => {
                // Code non reconnu
            }
        }

        self.pc += 2; // on avance l'index de 2 car chaque instruction prend une place de 2 cases
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
