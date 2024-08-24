use chip8_emulator::cpu::Chip8;
use eframe;
use eframe::egui::{self, Color32, Vec2};
use eframe::{run_native, NativeOptions};
use std::time::Duration;
use std::{
    fs::File,
    io::{BufReader, Read},
};

struct MyApp {
    cpu: Chip8,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(16));

        // Main display window
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let display = self.cpu.display;
            let (height, width) = (display[0].len(), display.len());
            let size = Vec2::new(20.0, 20.0); // Size of each "pixel"
            let origin = ui.min_rect().min; // Top-left corner of the painting area
            self.cpu.cycle();
            for y in 0..height {
                for x in 0..width {
                    let color = if display[x][y] {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    };

                    let pos = origin + Vec2::new(x as f32, y as f32) * size;
                    painter.rect_filled(egui::Rect::from_min_size(pos, size), 0.0, color);
                }
            }
        });

        // Second window with 16 buttons
        egui::Window::new("Control Panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                for i in 0..4 {
                    ui.horizontal(|ui| {
                        for j in 0..4 {
                            let button_label = format!("Button {:x}", i * 4 + j);
                            if ui.button(&button_label).clicked() {
                                println!("{button_label} was clicked!");
                                self.cpu.key = Some((i * 4 + j) as u8);
                            }
                        }
                    });
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    let mut cpu = Chip8::new();
    cpu.program_counter = 0x200;
    let mut buf = Vec::new();
    let mut rom =
        BufReader::new(File::open("./Jumping X and O [Harry Kleinberg, 1977].ch8").unwrap());
    let size = rom.read_to_end(&mut buf).unwrap();
    for i in 0..size as usize {
        cpu.memory[0x200 + i] = buf[i];
    }
    let app = MyApp { cpu };
    let options = NativeOptions::default();
    run_native("", options, Box::new(|_cc| Ok(Box::new(app))))
}
