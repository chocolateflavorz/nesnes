use crate::{cpu::Cpu, mem::Mem};

pub struct Emu {
    pub cpu: Cpu,
    pub mem: Mem,
}

impl Default for Emu {
    fn default() -> Self {
        Emu {
            cpu: Cpu::default(),
            mem: Mem::default(),
        }
    }
}

impl Emu {
    pub fn load(&mut self, bin: Vec<u8>) {
        self.mem.copy_from_slice(0x8000, &bin);
    }
    pub fn run(&mut self) {
    }
}
