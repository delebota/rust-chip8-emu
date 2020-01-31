use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::process::exit;
use sdl2::{Sdl, EventPump};
use sdl2::keyboard::Keycode;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const DARK_GRAY: Color = Color::RGB(90, 90, 90);
const LIGHT_GRAY: Color = Color::RGB(170, 170, 170);

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Display {
    pub memory: [u8; 2048],
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let canvas = sdl_context.video().unwrap()
            .window("Chip8 Emu", 512, 256).build().unwrap()
            .into_canvas().accelerated().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            memory: [0; 2048],
            sdl_context: sdl_context,
            canvas: canvas,
            event_pump: event_pump
        }
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(LIGHT_GRAY);
        self.canvas.clear();

        self.canvas.set_draw_color(DARK_GRAY);
        for y in 0..32 {
            for x in 0..64 {
                if self.memory[x + (y * 64)] == 1 {
                    let result = self.canvas.fill_rect(Rect::new((x * 8) as i32, (y * 8) as i32, 8, 8));
                    if result.is_err() {
                        eprintln!("Error: {:?}", result.err());
                        exit(2);
                    }
                }
            }
        }

        self.canvas.present();
    }

    pub fn clear_screen(&mut self) {
        self.memory = [0; 2048];
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let rows = sprite.len();
        let mut collision = 0u8;

        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 as usize {
                let new_value = row >> (7 - i) as u8 & 0x01;
                if new_value == 1 {
                    let xi = (x + i) % WIDTH;
                    let yj = (y + j) % HEIGHT;
                    let old_value = self.memory[xi + yj * WIDTH] == 1;
                    if old_value {
                        collision = 1;
                    }

                    self.memory[xi + yj * WIDTH] = ((new_value == 1) ^ old_value) as u8;
                }
            }
        }

        return collision;
    }

    pub fn check_keys_pressed(&mut self, mut keys: [bool; 16]) -> [bool; 16] {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => exit(0),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), ..} => exit(0),
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num0), ..} => {
                    keys[0x0] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num1), ..} => {
                    keys[0x1] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num2), ..} => {
                    keys[0x2] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num3), ..} => {
                    keys[0x3] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num4), ..} => {
                    keys[0x4] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num5), ..} => {
                    keys[0x5] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num6), ..} => {
                    keys[0x6] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num7), ..} => {
                    keys[0x7] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num8), ..} => {
                    keys[0x8] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Num9), ..} => {
                    keys[0x9] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    keys[0xA] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::B), ..} => {
                    keys[0xB] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::C), ..} => {
                    keys[0xC] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    keys[0xD] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    keys[0xE] = true;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::F), ..} => {
                    keys[0xF] = true;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num0), ..} => {
                    keys[0x0] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num1), ..} => {
                    keys[0x1] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num2), ..} => {
                    keys[0x2] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num3), ..} => {
                    keys[0x3] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num4), ..} => {
                    keys[0x4] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num5), ..} => {
                    keys[0x5] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num6), ..} => {
                    keys[0x6] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num7), ..} => {
                    keys[0x7] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num8), ..} => {
                    keys[0x8] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::Num9), ..} => {
                    keys[0x9] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::A), ..} => {
                    keys[0xA] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::B), ..} => {
                    keys[0xB] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::C), ..} => {
                    keys[0xC] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::D), ..} => {
                    keys[0xD] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::E), ..} => {
                    keys[0xE] = false;
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::F), ..} => {
                    keys[0xF] = false;
                },
                _ => {},
            }
        }

        return keys;
    }
}