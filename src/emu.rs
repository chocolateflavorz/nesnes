use crate::cart::Rom;
use crate::{cpu::Cpu, mem::Mem};

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
    pub mem: Mem,
}

pub struct Stat {
    pub cycle_counter: u32,
    pub frame_counter: u32,
}

impl Default for Emu {
    fn default() -> Self {
        Emu {
            stat: Stat::new(),
            cpu: Cpu::default(),
            mem: Mem::default(),
        }
    }
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            cycle_counter: 0,
            frame_counter: 0,
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
                panic!("error when loading rom : {}", e);
            }
        }
    }
}
