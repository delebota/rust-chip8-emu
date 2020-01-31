use crate::display::{Display, FONT_SET};
use crate::keypad::Keypad;
use std::fs::File;
use std::io;
use std::io::Read;
use rand::Rng;

pub struct CPU {
    pub memory: [u8; 4096],   // memory
    pub v: [u8; 16],          // registers
    pub index: u16,           // index register
    pub program_counter: u16, // program counter
    pub delay_timer: u8,      // delay timer
    pub sound_timer: u8,      // sound timer
    pub stack: [u16; 16],     // stack
    pub stack_pointer: u8,    // stack pointer
    pub draw_flag: bool,      // draw flag, signals if screen needs to be redrawn or not
    pub keypad: Keypad,
    pub display: Display
}

impl CPU {
    pub fn execute_cycle(&mut self) {
        // Fetch Opcode
        let opcode = (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[(self.program_counter + 1) as usize] as u16);

        // Decode & Execute Opcode
        self.process_opcode(opcode);

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0 {
                println!("*** BEEP ***");
            }
        }
    }

    fn process_opcode(&mut self, opcode: u16) {
        // extract common opcode parameters
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 =  opcode & 0x000F;

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xE, 0) => { //00E0
                self.display.clear_screen();
                self.program_counter += 2;
            },
            (0, 0, 0xE, 0xE) => { //00EE
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.program_counter += 2;
            },
            (0x1, _, _, _) => { //1NNN
                self.program_counter = nnn;
            },
            (0x2, _, _, _) => { //2NNN
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = nnn;
            },
            (0x3, _, _, _) => { //3XNN
                if vx == nn {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            (0x4, _, _, _) => { //4XNN
                if vx != nn {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            (0x5, _, _, 0) => { //5XY0
                if vx == vy {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            (0x6, _, _, _) => { //6XNN
                self.v[x] = nn;
                self.program_counter += 2;
            },
            (0x7, _, _, _) => { //7XNN
                let (res, _overflow) = self.v[x].overflowing_add(nn);
                self.v[x] = res;
                self.program_counter += 2;
            },
            (0x8, _, _, 0) => { //8XY0
                self.v[x] = vy;
                self.program_counter += 2;
            },
            (0x8, _, _, 1) => { //8XY1
                self.v[x] = vx | vy;
                self.program_counter += 2;
            },
            (0x8, _, _, 2) => { //8XY2
                self.v[x] = vx & vy;
                self.program_counter += 2;
            },
            (0x8, _, _, 3) => { //8XY3
                self.v[x] = vx ^ vy;
                self.program_counter += 2;
            },
            (0x8, _, _, 4) => { //8XY4
                let (res, overflow) = vx.overflowing_add(vy);
                match overflow {
                    true => self.v[0xF] = 1,
                    false => self.v[0xF] = 0,
                }
                self.v[x] = res;
                self.program_counter += 2;
            },
            (0x8, _, _, 5) => { //8XY5
                let (res, overflow) = vx.overflowing_sub(vy);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
                self.program_counter += 2;
            },
            (0x8, _, _, 6) => { //8XY6
                self.v[0xF] = vx & 1;
                self.v[x] = vx >> 1;
                self.program_counter += 2;
            },
            (0x8, _, _, 7) => { //8XY7
                let (res, overflow) = vy.overflowing_sub(vx);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
                self.program_counter += 2;
            },
            (0x8, _, _, 0xE) => { //8XYE
                self.v[0xF] = vx & 128;
                self.v[x] = vx << 1;
                self.program_counter += 2;
            },
            (0x9, _, _, 0) => { //9XY0
                if vx != vy {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            (0xA, _, _, _) => { //ANNN
                self.index = nnn;
                self.program_counter += 2;
            },
            (0xB, _, _, _) => { //BNNN
                self.program_counter = nnn + self.v[0] as u16;
            },
            (0xC, _, _, _) => { //CXNN
                let mut rng = rand::thread_rng();
                self.v[x] = rng.gen_range(0, 255) & nn;
                self.program_counter += 2;
            },
            (0xD, _, _, _) => { //DXYN
                self.v[0xF] = self.display.draw(vx as usize, vy as usize, &self.memory[self.index as usize .. (self.index + n as u16) as usize]);
                self.draw_flag = true;
                self.program_counter += 2;
            },
            (0xE, _, 9, 0xE) => { //EX9E
                if self.keypad.keys[vx as usize] == true {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            (0xE, _, 0xA, 1) => { //EXA1
                if self.keypad.keys[vx as usize] != true {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            (0xF, _, 0, 7) => { //FX07
                self.v[x] = self.delay_timer;
                self.program_counter += 2;
            },
            (0xF, _, 0, 0xA) => { //FX0A
                self.v[x] = self.keypad.wait_for_keypress();
                self.program_counter += 2;
            },
            (0xF, _, 1, 5) => { //FX15
                self.delay_timer = vx;
                self.program_counter += 2;
            },
            (0xF, _, 1, 8) => { //FX18
                self.sound_timer = vx;
                self.program_counter += 2;
            },
            (0xF, _, 1, 0xE) => { //FX1E
                if (self.index + vx as u16) > 0xFFF {
                    self.v[0xF] = 1; // Carry Flag
                } else {
                    self.v[0xF] = 0;
                }

                self.index += vx as u16;
                self.program_counter += 2;
            },
            (0xF, _, 2, 9) => { //FX29
                self.index = (vx as u16) * 5;
                self.program_counter += 2;
            },
            (0xF, _, 3, 3) => { //FX33
                self.memory[self.index as usize]     =  vx / 100;       // Set hundred's place
                self.memory[self.index as usize + 1] = (vx / 10 ) % 10; // Set ten's place
                self.memory[self.index as usize + 2] = (vx % 100) % 10; // Set one's place
                self.program_counter += 2;
            },
            (0xF, _, 5, 5) => { //FX55
                for i in 0..=x {
                    self.memory[(self.index + i as u16) as usize] = self.v[i as usize];
                }
                self.program_counter += 2;
            },
            (0xF, _, 6, 5) => { //FX65
                for i in 0..=x {
                    self.v[i as usize] = self.memory[(self.index + i as u16) as usize];
                }
                self.program_counter += 2;
            },
            (_, _, _, _) => println!("Unknown OpCode: {:#X}", opcode)
        }
    }

    pub fn load_fontset(&mut self) {
        for i in 0..80 {
            self.memory[i] = FONT_SET[i];
        }
    }

    pub fn load_program(&mut self, program: &str) -> io::Result<()> {
        let mut file = File::open(format!("roms/{}", program))?;
        let mut buffer = Vec::new();
        let file_size = file.read_to_end(&mut buffer)?;

        for i in 0..file_size {
            self.memory[0x200 + i] = buffer[i];
        }

        Ok(())
    }

    pub fn set_keys(&mut self) {
        let mut keys = self.keypad.keys;
        self.keypad.keys = self.display.check_keys_pressed(keys);
    }
}

