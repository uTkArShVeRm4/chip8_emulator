use core::panic;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub display: [bool; 32 * 16],
    pub stack: [u16; 12],
    pub program_counter: usize,
    pub stack_pointer: usize,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0u8; 4096],
            registers: [0u8; 16],
            display: [false; 32 * 16],
            stack: [0u16; 12],
            program_counter: 0usize,
            stack_pointer: 0usize,
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

    pub fn cycle(&mut self) {
        let opcode = self.read_opcode();
        self.increment_pc();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        let nnn = opcode & 0x0FFF;

        match (c, x, y, d) {
            (0, 0, 0xE, 0) => self.clear_display(),
            (0x1, _, _, _) => self.jump_nnn(nnn),
            (0x2, _, _, _) => self.call_subroutine(nnn),
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => (),
        }
    }

    pub fn clear_display(&mut self) {
        for i in 0..32 * 16 {
            self.display[i as usize] = false;
        }
    }

    pub fn jump_nnn(&mut self, nnn: u16) {
        // Move program counter to address nnn
        self.program_counter = nnn as usize;
    }

    pub fn call_subroutine(&mut self, nnn: u16) {
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
}
