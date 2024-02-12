use std::error::Error;

use sdl2::{pixels, rect::Rect, render::Canvas, video::Window};

use crate::{CHIP8_HEIGHT, CHIP8_WIDTH};

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("CHIP8 Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        canvas.set_draw_color(pixels::Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(DisplayDriver { canvas })
    }

    pub fn draw(&mut self, vram: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) -> Result<(), Box<dyn Error>> {
        for (y, row) in vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(match col {
                    0 => pixels::Color::BLACK,
                    _ => pixels::Color::GREEN,
                });
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))?;
            }
        }
        self.canvas.present();
        Ok(())
    }
}
