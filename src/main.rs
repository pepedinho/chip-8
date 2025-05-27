use std::{
    env::args,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use clap::Parser;
use cpu::schema::{Jump, Keyboard, CPU, CPU_SPEED};
use display::schema::{ContextPixels, Renderer, HEIGHT, WIDHT};
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
}

pub enum Order {
    Clear,
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
    let j = Jump::new();

    let mut renderer = Renderer::init(canvas, &texture_creator);

    let mut cpu = CPU::new(config.debug);
    match cpu.load_game(&config.rom_path) {
        Ok(()) => println!("Game was loaded succesfully !"),
        Err(e) => {
            println!("An error has occured during loading game : {}", e);
            return Ok(());
        }
    }
    cpu.init_memory(); //mapper la police

    let cpu = Arc::new(Mutex::new(cpu));
    let ctx = Arc::new(Mutex::new(ContextPixels::init()));
    let (tx, rx): (Sender<Order>, Receiver<Order>) = channel();

    let ctx_thread = Arc::clone(&ctx);

    thread::spawn(move || loop {
        for _ in 0..config.speed {
            let opcode;
            {
                let mut cpu = cpu.lock().unwrap();
                opcode = cpu.get_opcode();
            }
            {
                let mut cpu = cpu.lock().unwrap();
                let mut ctx = ctx_thread.lock().unwrap();
                cpu.interpret(opcode, &j, &mut ctx, &tx);
            }
        }
        {
            cpu.lock().unwrap().countdown();
        }
        thread::sleep(Duration::from_millis(16));
    });

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
                        let mut ctx = ctx.lock().unwrap();
                        ctx.keyboard.set_key(chip8_key, true);
                    }
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(chip8_key) = Keyboard::map_sdl_key_to_chip8(keycode) {
                        let mut ctx = ctx.lock().unwrap();
                        ctx.keyboard.set_key(chip8_key, false);
                    }
                }
                _ => {}
            }
        }

        while let Ok(msg) = rx.try_recv() {
            let mut ctx = ctx.lock().unwrap();
            match msg {
                Order::Clear => ctx.clear_screen(&mut renderer),
                //
            }
        }

        let mut ctx = ctx.lock().unwrap();
        ctx.update_screen(&mut renderer);

        // for _ in 0..config.speed {
        //     let opcode = cpu.get_opcode();
        //     cpu.interpret(opcode, &j, &mut ctx);
        // }
        //
        // ctx.update_screen();
        // cpu.countdown();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
