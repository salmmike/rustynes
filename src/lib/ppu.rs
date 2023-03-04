extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{Rectangle, rectangle, DrawState};
use opengl_graphics::GlGraphics;
use piston::{Key, Button};
use piston_window::{RenderArgs, clear, Context};


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
            /// PPU address
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
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