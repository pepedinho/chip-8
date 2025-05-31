use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

use crate::jit_compile_and_run;

use crate::display::schema::ContextPixels;
use dynasmrt::x64::Assembler;
use rand::random;

use super::schema::{
    JitBlock, Jump, Keyboard, CHIP8_FONTSET, CPU, MEM_SIZE, NBR_OPCODE, START_ADRR,
};

type JitFn = extern "C" fn(&mut CPU);

impl CPU {
    pub fn new(debug: bool) -> Self {
        let mut asms = Vec::with_capacity(35);
        for _ in 0..35 {
            asms.push(Some(Assembler::new().unwrap()));
        }
        CPU {
            mem: [0u8; MEM_SIZE],
            V: [0u8; 16],
            stack: [0u16; 16],
            pc: START_ADRR as u16,
            sp: 0,
            game_count: 0,
            sound_count: 0,
            I: 0,
            debug,
            jit_cache: HashMap::new(),
            asm: asms.try_into().unwrap(),
        }
    }

    pub fn init_memory(&mut self) {
        for i in 0..CHIP8_FONTSET.len() {
            self.mem[i] = CHIP8_FONTSET[i];
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

    pub fn interpret(&mut self, opcode: u16, j: &Jump, display: &mut ContextPixels) {
        // recuperation des sous partie de lopcode
        let b3 = ((opcode & (0x0F00)) >> 8) as u8; // on prend les 4 bits, b3 représente X
        let b2 = ((opcode & (0x00F0)) >> 4) as u8; // idem, b2 représente Y
        let b1 = (opcode & (0x000F)) as u8; // on prend les 4 bits de poids faible
        let nnn = opcode & 0x0FFF; // les 12 dernier bits | 4 premier = [numeros d'instruction (2)] | 12 dernier = [adrr]
        let kk = (opcode & 0x00FF) as u8; // les 8 derniers bits
        let mut can_iter = true;

        let b4 = j.get_action(opcode);

        if self.debug {
            println!(
                "PC={:03X} I={:03X} Opcode={:04X} V={:?} SP={} Stack={:?} Action={} | X = {} | Y = {}",
                self.pc,
                self.I,
                opcode,
                self.V,
                self.sp,
                &self.stack[..self.sp as usize],
                b4,
                b3,
                b2
            );
        }
        if let Some(jit_block) = self.jit_cache.get(&opcode) {
            let func: extern "C" fn(&mut CPU, &mut ContextPixels) -> bool = unsafe {
                std::mem::transmute(
                    jit_block
                        .code
                        .ptr(dynasmrt::AssemblyOffset(jit_block.entry)),
                )
            };
            can_iter = func(self, display);
        } else {
            match b4 {
                0 => {}                      // opcode non implementer
                1 => display.clear_screen(), // efface l'ecran
                2 => {
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_00EE);
                }
                3 => {
                    can_iter =
                        jit_compile_and_run!(self, display, opcode, CPU::jit_compile_1NNN, nnn);
                }
                4 => {
                    // 2NNN appelle le sous-programme en NNN, mais on revient ensuite.
                    can_iter =
                        jit_compile_and_run!(self, display, opcode, CPU::jit_compile_2NNN, nnn);
                }
                5 => {
                    // 3XKK saute l'instruction suivante si VX est égal à KK.
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_3XKK, b3, kk);
                }
                6 => {
                    // 4XKK saute l'instruction suivante si VX et KK ne sont pas égaux.
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_4XKK, b3, kk);
                }
                7 => {
                    // 5XY0 saute l'instruction suivante si VX et VY sont égaux.
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_5XY0, b3, b2);
                }
                8 => {
                    // 6XNN définit VX à KK.
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_6XNN, b3, kk);
                }
                9 => {
                    // 7XNN ajoute KK à VX.
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_7XNN, b3, kk);
                }
                10 => {
                    // 8XY0 définit VX à la valeur de VY.
                    //self.V[b3 as usize] = self.V[b2 as usize];
                    jit_compile_and_run!(self, display, opcode, CPU::jit_compile_8XY0, b3, b2);
                }
                11 => {
                    // 8XY1 définit VX à VX OR VY.
                    //println!("11");
                    self.V[b3 as usize] = self.V[b3 as usize] | self.V[b2 as usize];
                }
                12 => {
                    // 8XY2 définit VX à VX AND VY.
                    //println!("12");
                    self.V[b3 as usize] = self.V[b3 as usize] & self.V[b2 as usize];
                }
                13 => {
                    // 8XY3 définit VX à VX XOR VY.
                    //println!("13");
                    self.V[b3 as usize] = self.V[b3 as usize] ^ self.V[b2 as usize];
                }
                14 => {
                    // 8XY4 ajoute VY à VX. VF est mis à 1 quand il y a un dépassement de mémoire (carry), et à 0 quand il n'y en pas.
                    //println!("14");
                    let (result, carry) = self.V[b3 as usize].overflowing_add(self.V[b2 as usize]);
                    self.V[b3 as usize] = result;
                    self.V[0xF] = if carry { 1 } else { 0 };
                }
                15 => {
                    // 8XY5 VY est soustraite de VX. VF est mis à 0 quand il y a un emprunt, et à 1 quand il n'y a en pas.
                    //println!("15");
                    let (result, borrow) = self.V[b3 as usize].overflowing_sub(self.V[b2 as usize]);
                    self.V[b3 as usize] = result;
                    self.V[0xF] = if borrow { 0 } else { 1 };
                }
                16 => {
                    // 8XY6 décale (shift) VX à droite de 1 bit. VF est fixé à la valeur du bit de poids faible de VX avant le décalage.
                    //println!("16");
                    self.V[0xF] = self.V[b3 as usize] & 0x1;

                    self.V[b3 as usize] = self.V[b3 as usize] >> 1;
                }
                17 => {
                    // 8XY7 VX = VY - VX. VF est mis à 0 quand il y a un emprunt et à 1 quand il n'y en a pas.
                    //println!("17");
                    let (result, borrow) = self.V[b2 as usize].overflowing_sub(self.V[b3 as usize]);
                    self.V[b3 as usize] = result;
                    self.V[0xF] = if borrow { 0 } else { 1 };
                }
                18 => {
                    // 8XYE décale (shift) VX à gauche de 1 bit. VF est fixé à la valeur du bit de poids fort de VX avant le décalage.
                    //println!("18");
                    self.V[0xF] = (self.V[b3 as usize] >> 7) & 0x1;

                    self.V[b3 as usize] = self.V[b3 as usize] << 1;
                }
                19 => {
                    // 9XY0 saute l'instruction suivante si VX et VY ne sont pas égaux.
                    //println!("19");
                    if self.V[b3 as usize] != self.V[b2 as usize] {
                        self.pc += 2;
                    }
                }
                20 => {
                    // ANNN affecte NNN à I.
                    //println!("20");
                    self.I = nnn;
                }
                21 => {
                    // BNNN passe à l'adresse NNN + V0.
                    //println!("21");
                    self.pc = self.V[0] as u16 + nnn;
                }
                22 => {
                    // CXNN définit VX à un nombre aléatoire inférieur à NN.
                    //println!("22");
                    let r: u8 = random();
                    self.V[b3 as usize] = r & kk;
                }
                23 => {
                    // DXYN dessine un sprite aux coordonnées (VX, VY).
                    //println!("23");
                    display.draw_screen(b1, b3, b2, self);
                }
                24 => {
                    // EX9E saute l'instruction suivante si la clé stockée dans VX est pressée.
                    //println!("24");
                    let key = self.V[b3 as usize];

                    if display.keyboard.ispressed(key) {
                        self.pc += 2;
                    }
                }
                25 => {
                    // EXA1 saute l'instruction suivante si la clé stockée dans VX n'est pas pressée.
                    //println!("25");
                    let key = self.V[b3 as usize];

                    if !display.keyboard.ispressed(key) {
                        self.pc += 2;
                    }
                }
                26 => {
                    // FX07 définit VX à la valeur de la temporisation.
                    //println!("26");
                    self.V[b3 as usize] = self.game_count;
                }
                27 => {
                    // FX0A attend l'appui sur une touche et la stocke ensuite dans VX.
                    //println!("27");
                    can_iter = false;
                    if let Some(index) = display.keyboard.awaiting_key {
                        for (i, &pressed) in display.keyboard.keys.iter().enumerate() {
                            if pressed {
                                self.V[index as usize] = i as u8;
                                display.keyboard.awaiting_key = None;
                                can_iter = true;
                                break;
                            }
                        }
                    } else {
                        display.keyboard.awaiting_key = Some(b3);
                    }
                }
                28 => {
                    // FX15 définit la temporisation à VX.
                    //println!("28");
                    self.game_count = self.V[b3 as usize];
                }
                29 => {
                    // FX18 définit la minuterie sonore à VX.
                    //println!("29");
                    self.sound_count = self.V[b3 as usize];
                }
                30 => {
                    // FX1E ajo ute à VX I. VF est mis à 1 quand il y a overflow (I+VX>0xFFF), et à 0 si tel n'est pas le cas.
                    //println!("30");
                    let vx = self.V[b3 as usize] as u16;
                    let res = self.I + vx;

                    if res > 0x0FFF {
                        self.V[0xF] = 1;
                    } else {
                        self.V[0xF] = 0;
                    }
                    self.I = res;
                }
                31 => {
                    // FX29 définit I à l'emplacement du caractère stocké dans VX. Les caractères 0-F (en hexadécimal) sont représentés par une police 4x5.
                    //println!("31");
                    let digit = self.V[b3 as usize] as u16;
                    self.I = digit * 5;
                }
                32 => {
                    // FX33 stocke dans la mémoire le code décimal représentant VX (dans I, I+1, I+2).
                    //println!("32");
                    let value = self.V[b3 as usize];
                    self.mem[self.I as usize] = value / 100;
                    self.mem[(self.I + 1) as usize] = (value / 10) % 10;
                    self.mem[(self.I + 2) as usize] = value % 10;
                }
                33 => {
                    // FX55 stocke V0 à VX en mémoire à partir de l'adresse I.
                    //println!("33");
                    for i in 0..=b3 {
                        self.mem[(self.I + i as u16) as usize] = self.V[i as usize];
                    }
                }
                34 => {
                    // FX65 remplit V0 à VX avec les valeurs de la mémoire à partir de l'adresse I.
                    //println!("34");
                    for i in 0..=b3 {
                        self.V[i as usize] = self.mem[(self.I + i as u16) as usize];
                    }
                }
                _ => {
                    // Code non reconnu
                    println!("ERROR UNEXPECTED INSTRUCTION => ");
                }
            }
        }

        if can_iter {
            self.pc += 2; // on avance l'index de 2 car chaque instruction prend une place de 2 cases
        }
    }

    pub fn load_game(&mut self, path: &str) -> io::Result<()> {
        let mut game = File::open(path)?;
        let mut buffer = Vec::new();
        game.read_to_end(&mut buffer)?;

        let end = START_ADRR + buffer.len();
        self.mem[START_ADRR..end].copy_from_slice(&buffer);
        Ok(())
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

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: [false; 16],
            awaiting_key: None,
        }
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        if key < 16 {
            self.keys[key as usize] = pressed;
        }
    }
    pub fn ispressed(&self, key: u8) -> bool {
        if key < 16 {
            self.keys[key as usize]
        } else {
            false
        }
    }

    pub fn map_sdl_key_to_chip8(keycode: sdl2::keyboard::Keycode) -> Option<u8> {
        match keycode {
            sdl2::keyboard::Keycode::Num1 => Some(0x1),
            sdl2::keyboard::Keycode::Num2 => Some(0x2),
            sdl2::keyboard::Keycode::Num3 => Some(0x3),
            sdl2::keyboard::Keycode::Num4 => Some(0xC),
            sdl2::keyboard::Keycode::Q => Some(0x4),
            sdl2::keyboard::Keycode::W => Some(0x5),
            sdl2::keyboard::Keycode::E => Some(0x6),
            sdl2::keyboard::Keycode::R => Some(0xD),
            sdl2::keyboard::Keycode::A => Some(0x7),
            sdl2::keyboard::Keycode::S => Some(0x8),
            sdl2::keyboard::Keycode::D => Some(0x9),
            sdl2::keyboard::Keycode::F => Some(0xE),
            sdl2::keyboard::Keycode::Z => Some(0xA),
            sdl2::keyboard::Keycode::X => Some(0x0),
            sdl2::keyboard::Keycode::C => Some(0xB),
            sdl2::keyboard::Keycode::V => Some(0xF),
            _ => None,
        }
    }
}
