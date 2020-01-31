mod cpu;
mod display;
mod keypad;

use cpu::CPU;
use crate::keypad::Keypad;
use crate::display::Display;
use std::process::exit;
use std::time::Instant;
use std::{thread, time};

fn main() {
    // Initialize the Chip8 system
    let mut cpu = CPU {
        memory: [0; 4096],
        v: [0; 16],
        index: 0,
        program_counter: 0x200,
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; 16],
        stack_pointer: 0,
        draw_flag: false,
        keypad: Keypad {
            keys: [false; 16]
        },
        display: Display::new()
    };

    cpu.load_fontset();

    // Load the program into memory
    let result = cpu.load_program("breakout.ch8");
    if result.is_err() {
        eprintln!("Error: {:?}", result.err());
        exit(1);
    }

    // Emulation loop
    loop {
        let cycle_start = Instant::now();

        // Emulate one cycle
        cpu.execute_cycle();

        // If the draw flag is set, update the screen
        if cpu.draw_flag {
            cpu.display.render();
            cpu.draw_flag = false;
        }

        // Store key press state (Press and Release)
        cpu.set_keys();

        // Limit to ~60Hz
        if cycle_start.elapsed().as_millis() < 16 {
            thread::sleep(time::Duration::from_millis((16 - cycle_start.elapsed().as_millis()) as u64));
        }
    }
}
