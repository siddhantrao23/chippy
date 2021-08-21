use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

// resolution of display
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Display  {
    memory: Vec<u8>
}

#[wasm_bindgen]
impl Display {
    pub fn new() -> Display {
        Display {
            memory: vec![0; 2048],
        }
    }

    fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        self.memory[x + y * WIDTH]
    }

    fn set_pixel(&mut self, x: usize, y: usize, val: u8) {
        self.memory[x + y * WIDTH] = val;
    }

    pub fn cls(&mut self) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.set_pixel(i, j, 0);
            }
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        for j in 0..sprite.len() {
            let row = sprite[j];
            for i in 0..8 {
                if row & (0x80 >> i) != 0 {
                    let xi = (x + i) % WIDTH;
                    let yj = (y + j) % HEIGHT;
                    let pixel = self.get_pixel(xi, yj);
                    if pixel == 1 {
                        collision = true;
                    }
                    self.set_pixel(xi, yj, pixel ^ 1);
                }
            }
        }
        collision
    }
}

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
