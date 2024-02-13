extern crate sdl2;
mod drivers;

use crate::drivers::DisplayDriver;
use sdl2::{event::Event, keyboard::Keycode};

const CHIP8_MEMORY_SIZE: u16 = 4096;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut display_driver = DisplayDriver::new(&sdl_context).map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let vbuf = [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT];
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        display_driver.draw(&vbuf).map_err(|e| e.to_string())?;
    }

    Ok(())
}
