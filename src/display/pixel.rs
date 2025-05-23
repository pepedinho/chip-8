use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
};

use crate::cpu::schema::{Keyboard, CPU, MEM_SIZE};

use super::schema::{ContextPixels, Pixel, BLACK, DIMPIXEL, H, W, WHITE};

impl Pixel {
    pub fn new(pos: Rect) -> Self {
        Pixel {
            color: BLACK,
            position: pos,
        }
    }
}

impl<'a> ContextPixels<'a> {
    pub fn init(
        screen: Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        // créer les surfaces et les convertir en textures
        let mut surf_black =
            Surface::new(DIMPIXEL as u32, DIMPIXEL as u32, PixelFormatEnum::RGB24).unwrap();
        surf_black.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();

        let mut surf_white =
            Surface::new(DIMPIXEL as u32, DIMPIXEL as u32, PixelFormatEnum::RGB24).unwrap();
        surf_white
            .fill_rect(None, Color::RGB(255, 255, 255))
            .unwrap();

        let tex_black = texture_creator
            .create_texture_from_surface(&surf_black)
            .expect("Erreur texture noire");

        let tex_white = texture_creator
            .create_texture_from_surface(&surf_white)
            .expect("Erreur texture blanche");

        // Initialisation pixels
        let mut pixel = [[Pixel::new(Rect::new(0, 0, DIMPIXEL as u32, DIMPIXEL as u32));
            H as usize]; W as usize];

        for x in 0..W {
            for y in 0..H {
                pixel[x as usize][y as usize]
                    .position
                    .set_x((x as i32) * (DIMPIXEL as i32));
                pixel[x as usize][y as usize]
                    .position
                    .set_y((y as i32) * (DIMPIXEL as i32));

                pixel[x as usize][y as usize].color = BLACK;
            }
        }

        Self {
            screen,
            textures: [tex_black, tex_white],
            pixel,
            keyboard: Keyboard::new(),
        }
    }

    pub fn draw_pixel(&mut self, pixel: &Pixel) {
        let texture = &self.textures[pixel.color as usize];

        self.screen
            .copy(&texture, None, Some(pixel.position))
            .expect("error during pixel render");
    }

    pub fn clear_screen(&mut self) {
        for x in 0..W {
            for y in 0..H {
                self.pixel[x as usize][y as usize].color = BLACK;
            }
        }

        self.screen.set_draw_color(Color::BLACK);
        self.screen.clear();
    }

    pub fn update_screen(&mut self) {
        for x in 0..W as usize {
            for y in 0..H as usize {
                self.draw_pixel(&self.pixel[x][y].clone());
            }
        }
        self.screen.present();
    }

    pub fn draw_screen(&mut self, n: u8, x: u8, y: u8, cpu: &mut CPU) {
        //let k: u8 = 0;

        cpu.V[0xF] = 0;

        for byte_index in 0..n {
            let sprite_addr = cpu.I.wrapping_add(byte_index as u16);
            if sprite_addr as usize > MEM_SIZE {
                break;
            }
            let sprite_byte = cpu.mem[sprite_addr as usize];
            let y_pos = ((cpu.V[y as usize] as usize + byte_index as usize) % H as usize) as usize;

            for bit_index in 0..8 {
                let x_pos = ((cpu.V[x as usize] as usize + bit_index) % W as usize) as usize;

                let bit = (sprite_byte >> (7 - bit_index)) & 1;
                if bit == 1 {
                    if self.pixel[x_pos][y_pos].color == WHITE {
                        cpu.V[0xF] = 1; // colision
                    }
                    self.pixel[x_pos][y_pos].color ^= 1;
                }
            }
        }
    }
}
