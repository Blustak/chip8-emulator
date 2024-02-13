use std::error::Error;

use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl};

use crate::{CHIP8_HEIGHT, CHIP8_WIDTH};

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = sdl_context.video()?;
        let canvas = video_subsystem
            .window("Rust CHIP8 emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .build()?;
        Ok(DisplayDriver { canvas })
    }

    pub fn draw(&mut self, vbuf: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) -> Result<(), Box<dyn Error>> {
        for (y, col) in vbuf.iter().enumerate() {
            for (x, row) in col.iter().enumerate() {
                self.canvas.set_draw_color(match row {
                    0 => sdl2::pixels::Color::BLACK,
                    _ => sdl2::pixels::Color::WHITE,
                });
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))?;
            }
        }
        self.canvas.present();

        Ok(())
    }
}
