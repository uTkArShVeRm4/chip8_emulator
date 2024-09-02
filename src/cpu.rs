use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Chip8 {
    pub memory: [u8; MEMORY_SIZE],
    pub registers: [u8; NUM_REGISTERS],
    pub display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub stack: [u16; STACK_SIZE],
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub index: usize,
    pub keys: [bool; NUM_KEYS],
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Chip8 {
            memory: [0; MEMORY_SIZE],
            registers: [0; NUM_REGISTERS],
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            stack: [0; STACK_SIZE],
            program_counter: 0, // Programs typically start at 0x200
            stack_pointer: 0,
            index: 0,
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };

        let fonts = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (idx, byte) in fonts.iter().enumerate() {
            chip.memory[idx] = *byte;
        }
        chip
    }

    fn increment_pc(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(2);
    }

    pub fn cycle(&mut self) {
        if self.program_counter >= MEMORY_SIZE - 1 {
            panic!("Program counter out of bounds");
        }

        let opcode = (self.memory[self.program_counter] as u16) << 8
            | self.memory[self.program_counter + 1] as u16;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match nn {
                0xE0 => self.clear_screen(),
                0xEE => self.return_from_subroutine(),
                _ => println!("Unknown opcode: {:X}", opcode),
            },
            0x1000 => self.jump(nnn),
            0x2000 => self.call_subroutine(nnn),
            0x3000 => self.skip_if_equal(x, nn),
            0x4000 => self.skip_if_not_equal(x, nn),
            0x5000 => self.skip_if_equal_reg(x, y),
            0x6000 => self.set_register(x, nn),
            0x7000 => self.add_to_register(x, nn),
            0x8000 => self.alu_operations(x, y, n),
            0x9000 => self.skip_if_not_equal_reg(x, y),
            0xA000 => self.set_index(nnn),
            0xB000 => self.jump_with_offset(nnn),
            0xC000 => self.random(x, nn),
            0xD000 => self.draw(x, y, n),
            0xE000 => match nn {
                0x9E => self.skip_if_key_pressed(x),
                0xA1 => self.skip_if_key_not_pressed(x),
                _ => println!("Unknown opcode: {:X}", opcode),
            },
            0xF000 => self.misc_operations(x, nn),
            _ => println!("Unknown opcode: {:X}", opcode),
        }

        self.update_timers();
    }

    fn clear_screen(&mut self) {
        self.display.fill(0);
        self.increment_pc();
    }

    fn return_from_subroutine(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
        self.increment_pc();
    }

    fn jump(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }

    fn call_subroutine(&mut self, addr: u16) {
        if self.stack_pointer >= STACK_SIZE {
            panic!("Stack overflow");
        }
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn skip_if_equal(&mut self, x: usize, value: u8) {
        if self.registers[x] == value {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn skip_if_not_equal(&mut self, x: usize, value: u8) {
        if self.registers[x] != value {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn skip_if_equal_reg(&mut self, x: usize, y: usize) {
        if self.registers[x] == self.registers[y] {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn set_register(&mut self, x: usize, value: u8) {
        self.registers[x] = value;
        self.increment_pc();
    }

    fn add_to_register(&mut self, x: usize, value: u8) {
        self.registers[x] = self.registers[x].wrapping_add(value);
        self.increment_pc();
    }

    fn alu_operations(&mut self, x: usize, y: usize, op: u8) {
        match op {
            0x0 => self.registers[x] = self.registers[y],
            0x1 => self.registers[x] |= self.registers[y],
            0x2 => self.registers[x] &= self.registers[y],
            0x3 => self.registers[x] ^= self.registers[y],
            0x4 => {
                let (sum, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = sum;
                self.registers[0xF] = overflow as u8;
            }
            0x5 => {
                let (diff, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = diff;
                self.registers[0xF] = !borrow as u8;
            }
            0x6 => {
                self.registers[0xF] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            }
            0x7 => {
                let (diff, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = diff;
                self.registers[0xF] = !borrow as u8;
            }
            0xE => {
                self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
                self.registers[x] <<= 1;
            }
            _ => println!("Unknown ALU opcode: {:X}", op),
        }
        self.increment_pc();
    }

    fn skip_if_not_equal_reg(&mut self, x: usize, y: usize) {
        if self.registers[x] != self.registers[y] {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn set_index(&mut self, value: u16) {
        self.index = value as usize;
        self.increment_pc();
    }

    fn jump_with_offset(&mut self, addr: u16) {
        self.program_counter = (addr as usize + self.registers[0] as usize) & 0xFFF;
    }

    fn random(&mut self, x: usize, mask: u8) {
        let random: u8 = rand::thread_rng().gen();
        self.registers[x] = random & mask;
        self.increment_pc();
    }

    fn draw(&mut self, x: usize, y: usize, height: u8) {
        let x_coord = self.registers[x] as usize;
        let y_coord = self.registers[y] as usize;
        self.registers[0xF] = 0;

        for row in 0..height as usize {
            let sprite_byte = self.memory[self.index + row];
            for col in 0..8 {
                if (sprite_byte & (0x80 >> col)) != 0 {
                    let pixel_x = (x_coord + col) % DISPLAY_WIDTH;
                    let pixel_y = (y_coord + row) % DISPLAY_HEIGHT;
                    let pixel_index = pixel_y * DISPLAY_WIDTH + pixel_x;

                    if self.display[pixel_index] == 1 {
                        self.registers[0xF] = 1;
                    }
                    self.display[pixel_index] ^= 1;
                }
            }
        }
        self.increment_pc();
    }

    fn skip_if_key_pressed(&mut self, x: usize) {
        if self.keys[self.registers[x] as usize] {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn skip_if_key_not_pressed(&mut self, x: usize) {
        if !self.keys[self.registers[x] as usize] {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn misc_operations(&mut self, x: usize, op: u8) {
        match op {
            0x07 => self.registers[x] = self.delay_timer,
            0x0A => {
                let mut key_pressed = false;
                for (i, &key) in self.keys.iter().enumerate() {
                    if key {
                        self.registers[x] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed {
                    return; // Don't increment PC, repeat instruction
                }
            }
            0x15 => self.delay_timer = self.registers[x],
            0x18 => self.sound_timer = self.registers[x],
            0x1E => {
                self.index = self.index.wrapping_add(self.registers[x] as usize);
                self.registers[0xF] = (self.index > 0xFFF) as u8;
            }
            0x29 => self.index = self.registers[x] as usize * 5,
            0x33 => {
                self.memory[self.index] = self.registers[x] / 100;
                self.memory[self.index + 1] = (self.registers[x] / 10) % 10;
                self.memory[self.index + 2] = self.registers[x] % 10;
            }
            0x55 => {
                for i in 0..=x {
                    self.memory[self.index + i] = self.registers[i];
                }
            }
            0x65 => {
                for i in 0..=x {
                    self.registers[i] = self.memory[self.index + i];
                }
            }
            _ => println!("Unknown misc opcode: {:X}", op),
        }
        self.increment_pc();
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            // TODO: Implement sound
        }
    }
}
