use drivers::DisplayDriver;
use sdl2::event::Event;

extern crate sdl2;

mod drivers;

const CHIP8_WIDTH: usize = 64; //Width of CHIP8 screen
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096; //Size of memory for a chip8 system.

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut display_driver = DisplayDriver::new(&sdl_context).map_err(|e| e.to_string())?;
    let mut vram = [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT];
    display_driver.draw(&vram).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}
