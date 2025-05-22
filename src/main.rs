use std::time::Duration;

use cpu::schema::{Jump, Keyboard, MEM_SIZE};
use display::schema::{ContextPixels, DIMPIXEL, HEIGHT, WIDHT};
use sdl2::{event::Event, keyboard::Keycode};

mod cpu;
mod display;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip8", WIDHT, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut ctx = ContextPixels::init(canvas, &texture_creator);

    let j = Jump::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(chip8_key) = Keyboard::map_sdl_key_to_chip8(keycode) {
                        ctx.keyboard.set_key(chip8_key, true);
                    }
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(chip8_key) = Keyboard::map_sdl_key_to_chip8(keycode) {
                        ctx.keyboard.set_key(chip8_key, false);
                    }
                }
                _ => {}
            }
        }

        ctx.update_screen();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
