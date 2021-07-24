use crate::keypad::Keypad;
use crate::display::Display;

pub struct Cpu {
    // index reg
    pub i: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 4096],
    // registers
    pub v: [u8; 16],
    // peripherals
    pub keypad: Keypad,
    pub display: Display,
    // stack
    pub stack: [u16; 16],
    // stack pointer
    pub sp: u8,
    // delay timer
    pub dt: u8,
    // sound timer
    pub st: u8,
}

fn read_word(memory: [u8; 4096], counter: u16) -> u16 {
    (memory[counter as usize] as u16) << 8
        | (memory[(counter+1) as usize] as u16)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            keypad: Keypad::new(),
            display: Display::new(),
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn execute_cycle(&mut self) {
        let opcode: u16 = read_word(self.memory, self.pc);
        self.process_opcode(opcode);
    }

    // fetch and decode
    pub fn process_opcode(&mut self, opcode: u16) {
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        match (op_1, op_2, op_3, op_4) {
            // clear screen
            (0, 0, 0xE, 0) => self.display.cls(),
            // return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.sp = self.sp - 1;
                self.pc = self.stack[self.sp as usize]
            },
            // jump to nnn
            (0x1, _, _, _) => self.pc = nnn,
            // call to nnn
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp + 1;
                self.pc = nnn;
            },
            // skip if equal Vx, nn
            (0x3, _, _, _) => if vx == nn { self.pc += 2; },
            // skip if not equal Vx, nn
            (0x4, _, _, _) => if vx != nn { self.pc += 2; },
            // skip if equal Vx, Vy
            (0x5, _, _, _) => if vx == vy { self.pc += 2; },
            // load Vx, nn
            (0x6, _, _, _) => self.v[x] = nn,
            // add Vx, nn
            (0x7, _, _, _) => self.v[x] = self.v[x] + nn,
            // load Vx, Vy
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],
            // or Vx, Vy
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            // and Vx, Vy
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            // xor Vx, Vy
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            // add Vx, Vy update Vf
            (0x8, _, _, 0x4) => {
                let(res, overflow) = self.v[x].overflowing_add(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
            },
            // sub Vx, Vy update Vf
            (0x8, _, _, 0x5) => {
                let(res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
            },
            // right shift Vx
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[0xF] = self.v[0xF] >> 1;
            },
            // sub Vy, Vx update Vf
            (0x8, _, _, 0x7) => {
                let(res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
            },
            // left shift Vx
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[0xF] = self.v[0xF] << 1;
            },
            // skip if not equal Vx, Vy
            (0x9, _, _, _) => if vx != vy { self.pc += 2; },
            // load i, nnn
            (0xA, _, _, _) => self.i = nnn,
            // jump nnn + v0
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
            // random
            // TODO: fix later
            (0xC, _, _, _) => self.v[x] = 0,//self.rand.random() as u8 & nn,
            // draw Vx, Vy
            (0xD, _, _, _) => {
                let collision = self.display.draw(vx as usize, vy as usize,
                                                  &self.memory[self.i as usize .. (self.i + n as u16) as usize]);
                self.v[0xF] = if collision { 0 } else { 1 };
            },
            // skip if equal key, Vx
            (0xE, _, 0x9, 0xE) => if self.keypad.is_keypress(vx) { self.pc += 2; },
            // skip if not equal key, Vx
            (0xE, _, 0xA, 0x1) => if !self.keypad.is_keypress(vx) { self.pc += 2; },
            // load Vx, dt
            (0xF, _, 0x0, 0x7) => self.v[x] = self.dt,
            // load Vx, keypress
            (0xF, _, 0x0, 0xA) => {
                self.pc = self.pc - 2;
                for (i, key) in self.keypad.keys.iter().enumerate() {
                    if *key == true {
                        self.v[x] = i as u8;
                        self.pc = self.pc + 2;
                    }
                }
            },
            // load dt, Vx
            (0xF, _, 0x1, 0x5) => self.dt = self.v[x],
            // load st, Vx
            (0xF, _, 0x1, 0x8) => self.st = self.v[x],
            // load F, Vx
            (0xF, _, 0x1, 0xE) => self.i = self.i + self.v[x] as u16,
            // load F, Vx
            (0xF, _, 0x2, 0x9) => self.i = vx as u16 * 5,
            // load b, Vx
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = (vx % 100) % 10;
            },
            // load [I], Vx
            (0xF, _, 0x5, 0x5) => self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]
                .copy_from_slice(&self.v[0..(x as usize + 1)]),
            // load Vx, [I]
            (0xF, _, 0x6, 0x5) =>  self.v[0..(x as usize + 1)]
                .copy_from_slice(&self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]),
            (_, _, _, _) => ()
        }
    }
}
