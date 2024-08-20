use chip8_emulator::cpu::Chip8;

fn main() {
    println!("Hello, world!");
    let mut cpu = Chip8::new();
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;
    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x15;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();
    println!(
        "5 - 10 = {}, borrow: {}",
        cpu.registers[0], cpu.registers[0xF]
    );

    println!("{:?}", cpu.registers);
}
