struct Mapper {

}

impl Mapper {
    pub fn new() -> Mapper {
        Mapper {  }
    }

}

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mapper: Mapper,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            rom: vec![0; 0x3FFF],
            ram: vec![0; 0xFFF],
            mapper: Mapper::new(),
        }
    }
    pub fn write(&mut self, i: usize, value: u8) {
        self.rom[i] = value;
    }

    pub fn read(&self, i: usize) -> u8 {
        return self.rom[i];
    }
}