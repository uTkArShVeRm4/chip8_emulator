use chip8_emulator::cpu::Chip8;

fn main() {
    println!("Hello, world!");
    let mut cpu = Chip8::new();
    cpu.registers[0] = 200;
    cpu.registers[1] = 200;

    cpu.program_counter = 0x200;
    cpu.memory[0x200] = 0x80;
    cpu.memory[0x201] = 0x14;

    cpu.cycle();
    println!("{:?}", cpu.registers);
}
