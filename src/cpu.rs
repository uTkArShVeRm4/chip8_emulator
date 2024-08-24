use core::panic;
use std::ops::{Shl, Shr};

use rand::Rng;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub display: [[bool; 32]; 64],
    pub stack: [u16; 12],
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub index: usize,
    pub running: bool,
    pub key: Option<u8>,
    pub dt: u8,
    pub st: u8,
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
            key: None,
            dt: 0,
            st: 0,
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

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        let nnn = opcode & 0x0FFF;

        let kk = (opcode & 0x00FF) as u8;

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
            (0x8, _, _, 0x6) => self.shr_xy(x, y),
            (0x8, _, _, 0x7) => self.subn_xy(x, y),
            (0x8, _, _, 0xE) => self.shl_xy(x, y),
            (0x9, _, _, 0x0) => self.skip_ne_xy(x, y),
            (0xA, _, _, _) => self.set_index_register(nnn),
            (0xB, _, _, _) => self.jump_nnn_v0(nnn),
            (0xC, _, _, _) => self.rnd(x, kk),
            (0xD, _, _, _) => self.draw(x, y, d), // d is n here, height of sprite
            (0xE, _, 0x9, 0xE) => self.skip_x_key(x), // d is n here, height of sprite
            (0xE, _, 0xA, 0x1) => self.skip_nx_key(x), // d is n here, height of sprite
            (0xF, _, 0x0, 0x7) => self.load_dt_in_x(x), // d is n here, height of sprite
            _ => (),
        }
    }

    pub fn clear_display(&mut self) {
        eprintln!("CLEAR DISPLAY");
        for i in 0..64 {
            for j in 0..32 {
                self.display[i as usize][j as usize] = false;
            }
        }
        self.increment_pc();
    }

    pub fn ret(&mut self) {
        eprintln!("RETURN");
        if self.program_counter == 0 {
            panic!("STACK UNDERFLOW");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
        self.increment_pc();
    }

    pub fn jump_nnn(&mut self, nnn: u16) {
        eprintln!("JUMP NNN, {:x}", &nnn);
        // Move program counter to address nnn
        self.program_counter = nnn as usize;
    }

    pub fn call_subroutine(&mut self, nnn: u16) {
        eprintln!("CALL SUBROUTINE");
        // Move program counter to address nnn
        // and put cuurent address on stack as return addr
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("STACK OVERFLOW");
        }

        self.stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = nnn as usize;
        self.increment_pc();
    }

    pub fn skip_e(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] == kk {
            self.program_counter += 2;
        }
        self.increment_pc();
    }

    pub fn skip_ne(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] != kk {
            self.program_counter += 2;
        }
        self.increment_pc();
    }

    pub fn skip_e_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.program_counter += 2;
        }
        self.increment_pc();
    }
    pub fn skip_ne_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.program_counter += 2;
        }
        self.increment_pc();
    }

    pub fn load_byte_in_x(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
        self.increment_pc();
    }

    pub fn add_x_kk(&mut self, x: u8, kk: u8) {
        eprintln!("ADD X KK {:x} {:x}", &x, &kk);
        let arg1 = self.registers[x as usize];
        let arg2 = kk;

        self.registers[x as usize] = arg1 + arg2;
        self.increment_pc();
    }

    pub fn load_y_in_x(&mut self, x: u8, y: u8) {
        eprintln!("LOAD Y in X");
        self.registers[x as usize] = self.registers[y as usize];
        self.increment_pc();
    }

    pub fn add_xy(&mut self, x: u8, y: u8) {
        eprintln!("ADD XY");
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (sum, overflow) = arg1.overflowing_add(arg2);
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize] = sum;
        self.increment_pc();
    }

    pub fn or_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 | arg2;
        self.increment_pc();
    }

    pub fn and_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 & arg2;
        self.increment_pc();
    }

    pub fn xor_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        self.registers[x as usize] = arg1 ^ arg2;
        self.increment_pc();
    }

    pub fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        if arg1 > arg2 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        let diff = arg1 - arg2;
        self.registers[x as usize] = diff;
        self.increment_pc();
    }

    pub fn shr_xy(&mut self, x: u8, _y: u8) {
        let arg1 = self.registers[x as usize];
        // let arg2 = self.registers[_y as usize];

        let mut shr = arg1.shr(1);

        if (arg1 & 0x01) == 0x01 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        shr /= 2;
        self.registers[x as usize] = shr;
        self.increment_pc();
    }
    pub fn subn_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        if arg2 > arg1 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize] = arg2 - arg1;

        self.increment_pc();
    }

    pub fn shl_xy(&mut self, x: u8, _y: u8) {
        let arg1 = self.registers[x as usize];
        let mut shl = arg1.shl(1);

        if (arg1 & 0xA0) == 0xA0 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        shl *= 2;
        self.registers[x as usize] = shl;
        self.increment_pc();
    }

    pub fn set_index_register(&mut self, nnn: u16) {
        eprintln!("SET I, {:x}", nnn);
        self.index = nnn as usize;
        self.increment_pc();
    }

    pub fn jump_nnn_v0(&mut self, nnn: u16) {
        self.program_counter = nnn as usize + self.registers[0] as usize;
    }

    pub fn rnd(&mut self, x: u8, kk: u8) {
        let mut rng = rand::thread_rng();
        let random: u8 = rng.gen();

        self.registers[x as usize] = kk & random;
        self.increment_pc();
    }

    pub fn draw(&mut self, x: u8, y: u8, n: u8) {
        eprintln!("DRAW {:x} {:x} {:x}", &x, &y, &n);
        let cx = self.registers[x as usize] as usize % 64;
        let cy = self.registers[y as usize] as usize % 32;

        for y in 0..n as usize {
            let pixels = self.memory[self.index + y];
            let bits = bytes_to_binary(&pixels);
            if (cy + y) >= 32 {
                break;
            }
            for x in 0..8 {
                if (cx + x) >= 64 {
                    break;
                }
                let bit = if bits[x] > 0 { true } else { false };
                self.display[cx + x][cy + y] = self.display[x][y] ^ bit;
                if self.display[cx + x][cy + y] == false {
                    self.registers[0xF] = 1;
                }
            }
        }

        self.increment_pc();
    }

    pub fn skip_x_key(&mut self, x: u8) {
        if let Some(k) = self.key {
            if k == self.registers[x as usize] {
                self.increment_pc();
            }
        }
        self.increment_pc();
    }

    pub fn skip_nx_key(&mut self, x: u8) {
        if let Some(k) = self.key {
            if k != self.registers[x as usize] {
                self.increment_pc();
            }
        }
        self.increment_pc();
    }

    pub fn load_dt_in_x(&mut self, x: u8) {
        self.registers[x as usize] = self.dt;
    }

    pub fn wait_for_key(&mut self, x: u8) {
        if let Some(k) = self.key {
            self.registers[x as usize] = k;
            self.increment_pc();
        }
    }

    pub fn set_dt(&mut self, x: u8) {
        self.dt = self.registers[x as usize];
    }

    pub fn set_st(&mut self, x: u8) {
        self.st = self.registers[x as usize];
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
