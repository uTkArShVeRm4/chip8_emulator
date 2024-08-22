use std::{
    fs::File,
    io::{BufReader, Read},
};

use chip8_emulator::cpu::Chip8;

// fn main() {
//     println!("Hello, world!");
// let mut cpu = Chip8::new();
// cpu.program_counter = 0x200;
//
// let fonts: [u8; 5 * 16] = [
//     0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
//     0x20, 0x60, 0x20, 0x20, 0x70, // 1
//     0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
//     0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
//     0x90, 0x90, 0xF0, 0x10, 0x10, // 4
//     0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
//     0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
//     0xF0, 0x10, 0x20, 0x40, 0x40, // 7
//     0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
//     0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
//     0xF0, 0x90, 0xF0, 0x90, 0x90, // A
//     0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
//     0xF0, 0x80, 0x80, 0x80, 0xF0, // C
//     0xE0, 0x90, 0x90, 0x90, 0xE0, // D
//     0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
//     0xF0, 0x80, 0xF0, 0x80, 0x80, // F
// ];
//
// for i in 0..5 * 16 as usize {
//     cpu.memory[0x50 + i] = fonts[i];
// }

// let mut buf = Vec::new();
// let mut rom = BufReader::new(File::open("./IBM Logo.ch8").unwrap());
// let size = rom.read_to_end(&mut buf).unwrap();
// for i in 0..size as usize {
//     cpu.memory[0x200 + i] = buf[i];
// }
// cpu.run();
// println!("{:?}", cpu.display);
// }

use eframe;
use eframe::egui::{self, Color32, Painter, Vec2};
use eframe::{run_native, NativeOptions};

struct MyApp {
    display: [[bool; 32]; 64],
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let (height, width) = (self.display[0].len(), self.display.len());
            let size = Vec2::new(10.0, 10.0); // Size of each "pixel"
            let origin = ui.min_rect().min; // Top-left corner of the painting area

            for y in 0..height {
                for x in 0..width {
                    let color = if self.display[x][y] {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };

                    let pos = origin + Vec2::new(x as f32, y as f32) * size;
                    painter.rect_filled(egui::Rect::from_min_size(pos, size), 0.0, color);
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let display = [[false; 32]; 64]; // Example array, replace with your actual data

    let app = MyApp { display };
    let options = NativeOptions::default();
    run_native("Chip8", options, Box::new(|_cc| Ok(Box::new(app))))
}
