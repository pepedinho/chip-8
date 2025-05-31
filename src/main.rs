use std::time::{Duration, Instant};

use clap::Parser;
use cpu::schema::{Jump, Keyboard, CPU, CPU_SPEED};
use display::schema::{ContextPixels, HEIGHT, WIDHT};
use sdl2::{event::Event, keyboard::Keycode};

mod cpu;
mod display;

#[derive(Parser, Debug)]
#[command(author, version, about = "Émulateur Chip-8 en Rust")]
pub struct Config {
    pub rom_path: String,
    #[arg(short, long, default_value_t = CPU_SPEED)]
    pub speed: usize,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
    #[arg(short, long)]
    pub bench: Option<u32>,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let config = Config::parse();

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
    let mut cpu = CPU::new(config.debug);

    match cpu.load_game(&config.rom_path) {
        Ok(()) => println!("Game was loaded succesfully !"),
        Err(e) => {
            println!("An error has occured during loading game : {}", e);
            return Ok(());
        }
    }
    cpu.init_memory(); //mapper la police

    let j = Jump::new();

    if let Some(bench) = config.bench {
        println!("Start Benchmark ...");
        let start = Instant::now();
        for _ in 0..bench {
            for _ in 0..config.speed {
                let opcode = cpu.get_opcode();
                cpu.interpret(opcode, &j, &mut ctx);
            }

            ctx.update_screen();
            cpu.countdown();
        }
        let elapsed = start.elapsed();
        let ips = (bench as f64 * config.speed as f64) / elapsed.as_secs_f64();
        println!("Executed {} instructions in {:?}", bench, elapsed);
        println!("Instructions per second: {:.2}", ips);
        return Ok(());
    }

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

        for _ in 0..config.speed {
            let opcode = cpu.get_opcode();
            cpu.interpret(opcode, &j, &mut ctx);
        }

        ctx.update_screen();
        cpu.countdown();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
