extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{Rectangle, rectangle, DrawState};
use opengl_graphics::GlGraphics;
use piston::{Key, Button};
use piston_window::{RenderArgs, clear, Context};


use crate::Bus;

const CONTROLLER: usize = 0x2000;
const MASK: usize = 0x2001;
const STATUS: usize = 0x2002;
const OAM_ADDRESS: usize = 0x2003;
const OAM_DATA: usize = 0x2004;
const SCROLL: usize = 0x2005;
const ADDRESS: usize = 0x2006;
const DATA: usize = 0x2007;
const OAM_DMA: usize = 0x4014;

pub struct PPU {
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
        }
    }

    pub fn controller(&self, bus: &Bus) -> u8 {
        bus.read(CONTROLLER)
    }
    pub fn mask(&self, bus: &Bus) -> u8 {
        bus.read(MASK)
    }
    pub fn scroll(&self, bus: &Bus) -> u8 {
        bus.read(SCROLL)
    }
    pub fn status(&self, bus: &Bus) -> u8 {
        bus.read(STATUS)
    }
    pub fn oam_address(&self, bus: &Bus) -> u8 {
        bus.read(OAM_ADDRESS)
    }
    pub fn oam_data(&self, bus: &Bus) -> u8 {
        bus.read(OAM_DATA)
    }

    pub fn tick(&mut self, bus: &mut Bus) {

    }
}

pub struct Screen {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new() -> Screen {
        Screen { grid: vec!(vec!(0; 240); 256), width: 256, height: 240 }
    }

    pub fn draw(&self) {

    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {

    }

}