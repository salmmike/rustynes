mod cpu;
mod ppu;
mod bus;
mod cartridge;

pub use cpu::CPU;
pub use ppu::PPU;
pub use cpu::Instruction;
pub use bus::Bus;
pub use cartridge::Cartridge;

pub struct NES {
    pub bus:Bus,
    pub cpu: CPU,
    pub ppu: PPU,
    clock_count: usize,

}

impl NES {
    pub fn new() -> NES {
        NES {
            bus: Bus::new(),
            cpu: CPU::new(),
            ppu: PPU::new(),
            clock_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        self.clock_count += 1;
        self.clock_count &= 0xFFFF;
    }

}
