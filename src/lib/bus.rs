pub struct Bus {
    pub ram: Vec<u8>
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: vec!(0; 0x1FFF),
        }
    }

    pub fn read_ram(&self, i: usize) -> u8 {
        return self.ram[i & 0x7FF];
    }

    pub fn write_ram(&mut self, i: usize, value: u8) {
        self.ram[i & 0x7FF] = value;
    }
}