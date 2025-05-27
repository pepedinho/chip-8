use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::cpu::schema::Keyboard;

pub const BLACK: u8 = 0;
pub const WHITE: u8 = 1;
pub const W: u32 = 64; // nombre de pixels suivant la largeur
pub const H: u32 = 32; // nombre de pixels suivant la longueur
pub const DIMPIXEL: u32 = 8; // pixel carre de cote 8
pub const WIDHT: u32 = W * DIMPIXEL; // largeur de l'écran
pub const HEIGHT: u32 = H * DIMPIXEL; // longueur de l'écran

#[derive(Clone, Copy)]
pub struct Pixel {
    pub position: Rect,
    pub color: u8,
    pub dirty: bool, // sert q indiquer si il y a eu un changement d'etat du pixel
}

pub struct ContextPixels<'a> {
    pub screen: Canvas<Window>,
    pub textures: [Texture<'a>; 2],
    pub pixel: [[Pixel; H as usize]; W as usize],
    pub keyboard: Keyboard,
}
