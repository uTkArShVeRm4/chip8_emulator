use chip8_emulator::cpu::Chip8;
use raylib::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let mut cpu = Chip8::new();
    cpu.program_counter = 0x200;

    // Load ROM file into memory
    let mut buf = Vec::new();
    let mut rom = BufReader::new(File::open("./Pong (1 player).ch8").unwrap());
    let size = rom.read_to_end(&mut buf).unwrap();
    for i in 0..size as usize {
        cpu.memory[0x200 + i] = buf[i];
    }

    let (mut rl, thread) = raylib::init()
        .size(1024, 512)
        .title("CHIP-8 Emulator")
        .build();
    rl.set_target_fps(360);

    // Create a texture buffer to store the display data
    let mut texture_data: [u32; 64 * 32] = [0; 64 * 32];

    while !rl.window_should_close() {
        cpu.cycle();

        // Manually update the texture buffer based on the CHIP-8 display memory
        for y in 0..32 {
            for x in 0..64 {
                let index = y * 64 + x;
                texture_data[index] = if cpu.display[index] == 1 {
                    0xFFFFFFFF // White color for 'on' pixels
                } else {
                    0x000000FF // Black color for 'off' pixels
                };
            }
        }

        // Start drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Render the texture buffer to the screen
        for y in 0..32 {
            for x in 0..64 {
                let color = if texture_data[y * 64 + x] == 0xFFFFFFFF {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                // Scale each pixel to 16x16 for visibility
                d.draw_rectangle(x as i32 * 16, y as i32 * 16, 16, 16, color);
            }
        }
    }
}
