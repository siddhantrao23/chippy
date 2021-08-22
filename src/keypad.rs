use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Keypad {
    keys: Vec<u8>
}

#[wasm_bindgen]
impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: vec![0; 16]
        }
    }

    pub fn key_down(&mut self, index: u8) {
        self.keys[index as usize] = 1;
    }

    pub fn key_up(&mut self, index: u8) {
        self.keys[index as usize] = 0;
    }

    pub fn is_key_down(&self, index: u8) -> bool {
        self.keys[index as usize] == 1
    }

}
