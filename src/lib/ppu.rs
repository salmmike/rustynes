use crate::Bus;

pub struct PPU {
    v: u16,
    t: u16,
    x: u8,
    w: bool,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
    }

    pub fn tick(&mut self, bus: &mut Bus) {

    }
}