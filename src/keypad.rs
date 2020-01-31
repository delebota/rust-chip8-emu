pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn wait_for_keypress(&mut self) -> u8 {
        return 0;
    }
}