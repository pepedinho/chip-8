pub const MEM_SIZE: usize = 4096;
pub const START_ADRR: u16 = 512;
pub const NBR_OPCODE: usize = 35;

#[allow(non_snake_case)]
pub struct CPU {
    pub mem: [u8; MEM_SIZE], // memoire
    pub V: [u8; 16],         // le registre
    pub I: u16,              // stock une adresse mémoire ou dessinateur
    pub jump: [u16; 16],     // pour gérer les sauts dans « mémoire », 16 au maximum
    pub jump_counter: u8,    // stock le nombre de sauts effectués pour ne pas dépasser 16
    pub game_count: u8,      // compteur pour la synchronisation
    pub sound_count: u8,     // compteur pour le son
    pub pc: u16,             // pour parcourir le tableau « mémoire »
}

pub struct Jump {
    pub mask: [u16; NBR_OPCODE],
    pub id: [u16; NBR_OPCODE],
}
