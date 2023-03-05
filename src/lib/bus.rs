use crate::Cartridge;

pub struct Bus {
    /// CPU ram
    pub ram: Vec<u8>,
    /// PPU registers
    pub ppu_mem: Vec<u8>,
    /// APU and I/O registers
    pub apu_io: Vec<u8>,
    pub apu_io_test: Vec<u8>,
    /// Cartridge space
    pub cartridge: Cartridge,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: vec!(0; 0x800),
            ppu_mem: vec!(0; 8),
            apu_io: vec!(0; 0x18),
            apu_io_test: vec!(0; 8),
            cartridge: Cartridge::new(),
        }
    }

    pub fn read(&self, i: usize) -> u8 {
        if i <= 0x1FFF {
            return self.ram[i & 0x7FF];
        } else if i <= 0x3FFF {
            return  self.ppu_mem[i & 0x7];
        } else if i <= 0x4017 {
            return self.apu_io[i % 0x17];
        } else if i <= 0x401F {
            return self.apu_io_test[i & 0x7];
        } else {
            return self.cartridge.read(i & 0x7FFF);
        }
    }

    pub fn write(&mut self, i: usize, value: u8) {
        if i <= 0x1FFF {
            self.ram[i & 0x7FF] = value;
        } else if i <= 0x3FFF {
            self.ppu_mem[i & 0x8] = value;
        } else if i <= 0x4017 {
            self.apu_io[i & 0x18] = value;
        } else if i <= 0x401F {
            self.apu_io_test[i & 0x8] = value;
        } else {
            self.cartridge.write(i & 0x7FFF, value);
        }
    }
}