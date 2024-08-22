use core::panic;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub display: [[bool; 32]; 64],
    pub stack: [u16; 12],
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub index: usize,
    pub running: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0u8; 4096],
            registers: [0u8; 16],
            display: [[false; 32]; 64],
            stack: [0u16; 12],
            program_counter: 0usize,
            stack_pointer: 0usize,
            index: 0usize,
            running: false,
        }
    }

    pub fn read_opcode(&self) -> u16 {
        let pos = self.program_counter;
        let op_byte1 = self.memory[pos] as u16;
        let op_byte2 = self.memory[pos + 1] as u16;
        // basically concatenating the two bytes
        op_byte1 << 8 | op_byte2
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.read_opcode();
        self.increment_pc();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        let nnn = opcode & 0x0FFF;

        let kk = (opcode & 0x00FF >> 8) as u8;

        match (c, x, y, d) {
            (0, 0, 0, 0) => {
                self.running = false;
                return;
            }
            (0, 0, 0xE, 0) => self.clear_display(),
            (0, 0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jump_nnn(nnn),
            (0x2, _, _, _) => self.call_subroutine(nnn),
            (0x3, _, _, _) => self.skip_e(x, kk),
            (0x4, _, _, _) => self.skip_ne(x, kk),
            (0x5, _, _, 0x0) => self.skip_e_xy(x, y),
            (0x6, _, _, _) => self.load_byte_in_x(x, kk),
            (0x7, _, _, _) => self.add_x_kk(x, kk),
            (0x8, _, _, 0x0) => self.load_y_in_x(x, y),
            (0x8, _, _, 0x1) => self.or_xy(x, y),
            (0x8, _, _, 0x2) => self.and_xy(x, y),
            (0x8, _, _, 0x3) => self.xor_xy(x, y),
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            (0x8, _, _, 0x5) => self.sub_xy(x, y),
            (0xA, _, _, _) => self.set_index_register(nnn),
            (0xD, _, _, _) => self.draw(x, y, d), // d is n here, height of sprite
            _ => (),
        }
    }

    pub fn clear_display(&mut self) {
        for i in 0..64 {
            for j in 0..32 {
                self.display[i as usize][j as usize] = false;
            }
        }
    }

    pub fn ret(&mut self) {
        if self.program_counter == 0 {
            panic!("STACK UNDERFLOW");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    pub fn jump_nnn(&mut self, nnn: u16) {
        // Move program counter to address nnn
        self.program_counter = nnn as usize;
    }

    pub fn call_subroutine(&mut self, nnn: u16) {
        // Move program counter to address nnn
        // and put cuurent address on stack as return addr
        println!("MOVING TO ADDR {:x}", &nnn);
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("STACK OVERFLOW");
        }

        self.stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = nnn as usize;
    }

    pub fn skip_e(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] == kk {
            self.program_counter += 2;
        }
    }

    pub fn skip_ne(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] != kk {
            self.program_counter += 2;
        }
    }

    pub fn skip_e_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == y {
            self.program_counter += 2;
        }
    }

    pub fn load_byte_in_x(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    pub fn add_x_kk(&mut self, x: u8, kk: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = kk;
        let (sum, overflow) = arg1.overflowing_add(arg2);
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize] = sum;
    }

    pub fn load_y_in_x(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    pub fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (sum, overflow) = arg1.overflowing_add(arg2);
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize] = sum;
    }

    pub fn or_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 | arg2;
    }

    pub fn and_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 & arg2;
    }

    pub fn xor_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 ^ arg2;
    }

    pub fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let diff;

        if arg1 > arg2 {
            self.registers[0xF] = 1;
            diff = arg1 - arg2;
        } else {
            self.registers[0xF] = 0;
            diff = arg2 - arg1;
        }
        self.registers[x as usize] = diff;
    }

    pub fn set_index_register(&mut self, nnn: u16) {
        self.index = nnn as usize;
    }

    pub fn draw(&mut self, vx: u8, vy: u8, n: u8) {
        let mut x = (self.registers[vx as usize] % 64) as usize;
        let mut y = (self.registers[vy as usize] % 32) as usize;
        self.registers[0xF] = 0;

        for i in 0..n as usize {
            let curr_byte = self.memory[self.index + i];
            let curr_bits = bytes_to_binary(&curr_byte);
            for bit in curr_bits {
                if self.display[x][y] == true && bit == 1 {
                    self.display[x][y] = false;
                    self.registers[0xF] = 1;
                } else if self.display[x][y] == false && bit == 0 {
                    self.display[x][y] = true;
                }

                x += 1;
                if x == 64 {
                    x = (self.registers[vx as usize] % 64) as usize;
                    break;
                }
            }

            y += 1;
            if y == 32 {
                break;
            }
        }
    }
}

fn bytes_to_binary(x: &u8) -> [u8; 8] {
    let mut bits = [0u8; 8];
    bits[0] = (x & 0b10000000) >> 7;
    bits[1] = (x & 0b1000000) >> 6;
    bits[2] = (x & 0b100000) >> 5;
    bits[3] = (x & 0b10000) >> 4;
    bits[4] = (x & 0b1000) >> 3;
    bits[5] = (x & 0b100) >> 2;
    bits[6] = (x & 0b10) >> 1;
    bits[7] = x & 0b1;

    bits
}
