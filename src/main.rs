use rustyneslib::NES;
use std::io::{stdin};

fn main() {
    let mut s=String::new();

    println!("Hello, world!");
    let mut nes = NES::new();
    let program = vec!(0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03,
                       0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00,
                       0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D,
                       0x02, 0x00, 0xEA, 0xEA, 0xEA);

    for i in 0..program.len() {
        nes.memory[0x8000 + i] = program[i];
    }
    nes.cpu.set_pc(0x8000);

    while true {
        nes.cpu.tick(&mut nes.memory);
        nes.cpu.print_page(&nes.memory, 0);
        nes.cpu.print_page(&nes.memory, 0x1000);
        nes.cpu.print_register();
        nes.cpu.print_status();
        while nes.cpu.cycles > 0 {
            nes.cpu.tick(&mut nes.memory);
        }
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "e" {
            break;
        }
    }

}
