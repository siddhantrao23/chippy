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
                self.stack[self.sp] = self.pc;
                self.sp = self.sp + 1;
                self.pc = nnn;
            },
            // skip if equal Vx, nn
            (0x3, _, _, _) => if(vx == nn) { self.pc += 1; } ,
            // skip if not equal Vx, nn
            (0x4, _, _, _) => if(vx != nn) { self.pc += 1; } ,
            // skip if equal Vx, Vy
            (0x5, _, _, _) => if(vx == vy) { self.pc += 1; } ,
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
            // load F, Vx
            (0xF, _, 0x2, 0x9) => self.i = vx as u16 u16 * 5,
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i + 1 as usize] = (vx / 10) % 10;
                self.memory[self.i + 2 as usize] = (vx % 100) % 10;
            },

            (_, _, _, _) => ()
        }
    }
}
