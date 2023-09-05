use crate::{cpu::Cpu, mem::Mem};

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
        self.mem.copy_from_slice(0x8000, &bin);
    }
    pub fn run(&mut self) {}
}
