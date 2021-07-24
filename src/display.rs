// resolution of display
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display  {
    pub memory: [u8; 2048],
}

impl Display {
    pub fn new() -> Display {
        Display {
            memory: [0; 2048],
        }
    }

    fn get_pixel(self, x: usize, y: usize) -> u8 {
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
                if row & (0x80 >> i) == 1 {
                    let pixel = self.get_pixel(x+i, y+j);
                    if pixel == 1 {
                        collision = true;
                    }
                    self.set_pixel(x+i, y+j, pixel ^ 1);
                }
            }
        }
        collision
    }
}
