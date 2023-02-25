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

    nes.cpu.set_ram(&mut nes.bus, &program, 0x500);
    nes.cpu.set_pc(0x500);

    loop {
        nes.cpu.tick(&mut nes.bus);
        nes.cpu.print_page(&nes.bus, 0);
        nes.cpu.print_page(&nes.bus, 0x500);
        nes.cpu.print_register();
        nes.cpu.print_status();
        while nes.cpu.cycles > 0 {
            nes.cpu.tick(&mut nes.bus);
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
