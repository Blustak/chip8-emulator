use std::error::Error;

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

pub enum InputResult {
    Exit(Result<(), Box<dyn Error>>),
    KeyMapUpdate,
}

pub struct InputDriver {
    events: EventPump,
    pub keymap: [bool; 16],
}

impl InputDriver {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let events = sdl_context.event_pump()?;

        Ok(InputDriver {
            events,
            keymap: [false; 16],
        })
    }

    pub fn poll(&mut self) -> InputResult {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return InputResult::Exit(Ok(())),
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    repeat: false,
                    ..
                } => {
                    self.keymap[0] = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    repeat: false,
                    ..
                } => self.keymap[0] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    repeat: false,
                    ..
                } => self.keymap[1] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    repeat: false,
                    ..
                } => self.keymap[1] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    repeat: false,
                    ..
                } => self.keymap[2] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    repeat: false,
                    ..
                } => self.keymap[2] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    repeat: false,
                    ..
                } => self.keymap[3] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    repeat: false,
                    ..
                } => self.keymap[3] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    repeat: false,
                    ..
                } => self.keymap[4] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    repeat: false,
                    ..
                } => self.keymap[4] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => self.keymap[5] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => self.keymap[5] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    repeat: false,
                    ..
                } => self.keymap[6] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    repeat: false,
                    ..
                } => self.keymap[6] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } => self.keymap[7] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    repeat: false,
                    ..
                } => self.keymap[7] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                } => self.keymap[8] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                } => self.keymap[8] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => self.keymap[0x9] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => self.keymap[0x9] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => self.keymap[0xa] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => self.keymap[0xa] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    repeat: false,
                    ..
                } => self.keymap[0xb] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    repeat: false,
                    ..
                } => self.keymap[0xb] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    repeat: false,
                    ..
                } => self.keymap[0xc] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    repeat: false,
                    ..
                } => self.keymap[0xc] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    repeat: false,
                    ..
                } => self.keymap[0xd] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    repeat: false,
                    ..
                } => self.keymap[0xd] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    repeat: false,
                    ..
                } => self.keymap[0xe] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    repeat: false,
                    ..
                } => self.keymap[0xe] = false,

                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    repeat: false,
                    ..
                } => self.keymap[0xf] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    repeat: false,
                    ..
                } => self.keymap[0xf] = false,

                _ => {}
            }
        }
        InputResult::KeyMapUpdate
    }
}
