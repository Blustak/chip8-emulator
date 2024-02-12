use drivers::DisplayDriver;
use drivers::{InputDriver, InputResult};
use sdl2::event::Event;

extern crate sdl2;

mod drivers;

const CHIP8_WIDTH: usize = 64; //Width of CHIP8 screen
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096; //Size of memory for a chip8 system.

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut display_driver = DisplayDriver::new(&sdl_context).map_err(|e| e.to_string())?;
    let mut input_driver = InputDriver::new(&sdl_context).map_err(|e| e.to_string())?;
    let mut vram = [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT];
    display_driver.draw(&vram).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        match input_driver.poll() {
            InputResult::Exit(_) => break 'running,
            _ => {}
        }
        let mut silly_col = [0u8; CHIP8_WIDTH];
        for (i, &val) in input_driver.keymap.iter().enumerate() {
            let v = val as u8;
            silly_col[i] = v;
            silly_col[i + 1] = v;
        }
        vram[0] = silly_col;
        display_driver.draw(&vram).map_err(|e| e.to_string())?;
    }
    Ok(())
}
