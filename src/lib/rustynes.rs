mod cpu;
mod ppu;
pub use cpu::CPU;
pub use ppu::PPU;
pub use cpu::Instruction;

pub struct NES {
    pub memory: Vec<u8>,
    pub cpu: CPU,
    pub ppu: PPU,

}

impl NES {
    pub fn new() -> NES {
        NES {
            memory: vec![0 as u8; 0xFFFF],
            cpu: CPU::new(),
            ppu: PPU::new(),
        }
    }
}
