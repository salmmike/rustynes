struct Mapper {
    /// Mapper type
    ntype: u8,
}

impl Mapper {
    /// Create type 0 mapper
    pub fn new() -> Mapper {
        Mapper {
            ntype: 0,
         }
    }

}

pub struct Cartridge {
    prg: Vec<u8>,
    chr: Vec<u8>,
    mapper: Mapper,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
        /// NROM
            prg: vec![0; 0x3FFF],
            chr: vec![0; 0x1FFF],
            mapper: Mapper::new(),
        }
    }

    pub fn load(&mut self, file_path: String) {

    }

    pub fn write(&mut self, i: usize, value: u8) {
        self.prg[i] = value;
    }

    pub fn read(&self, i: usize) -> u8 {
        return self.prg[i];
    }
}