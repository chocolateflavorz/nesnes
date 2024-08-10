use crate::cart::Rom;
use crate::cpu::Cpu;
use crate::mem::Mem;
use crate::ppu::Ppu;

use log::debug;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use pixels::{PixelsBuilder, SurfaceTexture};

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;
const SCALE: f64 = 30.0;

pub struct Emu {
    pub stat: Stat,
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub mem: Mem,
}

pub struct Stat {
    pub cycle_counter: u32,
    pub frame_counter: u32,
    pub operand: String,
}

impl Default for Emu {
    fn default() -> Self {
        let mem = Mem::new();
        Emu {
            stat: Stat::new(),
            cpu: Cpu::default(),
            ppu: Ppu::new( Box::new(mem)),
            mem,
        }
    }
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            cycle_counter: 0,
            frame_counter: 0,
            operand: String::with_capacity(8),
        }
    }
}

impl Emu {
    pub fn load(&mut self, bin: Vec<u8>) {
        match Rom::from_raw(&bin) {
            Ok(rom) => {
                self.mem.set_rom(rom);
            }
            Err(e) => {
                panic!("loading rom error. {}", e);
            }
        }
    }

    pub fn memory_map_to_foo(&self) {
        use pretty_hex::*;
        use std::io::Write;
        let mut file = std::fs::File::create("foo.txt").unwrap();
        file.write(format!("{:?}", self.mem.mem[0x000..0xffff].hex_dump()).to_string().as_bytes()).expect("FUCKWRITER");
    }
}
