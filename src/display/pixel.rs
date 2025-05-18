use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, TextureCreator},
    surface::Surface,
    sys::{SDL_FillRect, SDL_blit},
    video::{Window, WindowContext},
    Sdl,
};

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
        let mut pixel = [[Pixel {
            position: Rect::new(0, 0, DIMPIXEL as u32, DIMPIXEL as u32),
            color: 0,
        }; H as usize]; W as usize];

        for x in 0..W {
            for y in 0..H {
                pixel[x as usize][y as usize]
                    .position
                    .set_x((x as i32) * (DIMPIXEL as i32));
                pixel[x as usize][y as usize]
                    .position
                    .set_y((y as i32) * (DIMPIXEL as i32));
                pixel[x as usize][y as usize].color = 0;
            }
        }

        Self {
            screen,
            textures: [tex_black, tex_white],
            pixel,
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
}
