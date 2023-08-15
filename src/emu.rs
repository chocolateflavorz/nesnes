use crate::{cpu::Cpu, mem::Mem};

pub struct Emulator {
    cpu: Cpu,
    mem: Mem,
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            cpu: Cpu::default(),
            mem: Mem::default(),
        }
    }
}

impl Emulator {
    pub fn load(&mut self, bin: Vec<u8>) {
        self.mem.copy_from_slice(0x8000, &bin);
    }
    pub fn run(&mut self) {
    }
}
