mod cpu;
mod ppu;
mod bus;
pub use cpu::CPU;
pub use ppu::PPU;
pub use cpu::Instruction;
pub use bus::Bus;

pub struct NES {
    pub bus:Bus,
    pub cpu: CPU,
    pub ppu: PPU,

}

impl NES {
    pub fn new() -> NES {
        NES {
            bus: Bus::new(),
            cpu: CPU::new(),
            ppu: PPU::new(),
        }
    }
}
