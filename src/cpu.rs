use bitflags::bitflags;
use log::info;
use log::debug;

use crate::emu::Emu;
use crate::ops::OP_CYCLE;
use crate::ops::OP_FUNC;
use crate::ops::OP_NAME;
//use crate::ops::OP_NAME;

bitflags! {
    pub struct Flags: u8 {
        const N = 0b10000000;
        const V = 0b01000000;
        const A = 0b00100000;
        const B = 0b00010000;
        const D = 0b00001000;
        const I = 0b00000100;
        const Z = 0b00000010;
        const C = 0b00000001;
    }
}

pub struct Cpu {
    pub pc: u16,
    pub sp: Flags,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
}


impl Cpu {
    pub fn nz_flags(&mut self, val: u8) {
        self.sp.set(Flags::N, val & 0x80 != 0);
        self.sp.set(Flags::Z, val == 0);
    }
    pub fn carry_flag(&mut self, val: bool) {
        self.sp.set(Flags::C, val);
    }
    pub fn overflow_flag(&mut self, val: bool) {
        self.sp.set(Flags::V, val);
    }
    pub fn decimal_flag(&mut self, val: bool) {
        self.sp.set(Flags::D, val);
    }
    pub fn interrupt_flag(&mut self, val: bool) {
        self.sp.set(Flags::I, val);
    }
    pub fn negative_flag(&mut self, val: bool) {
        self.sp.set(Flags::N, val);
    }
    pub fn zero_flag(&mut self, val: bool) {
        self.sp.set(Flags::Z, val);
    }
    pub fn stack_ptr(&self) -> u16 {
        self.s as u16 + 0x0100
    }
}

impl Emu {
    pub fn cpuinfo(emu: &Emu) {
        let op = emu.mem.load_u8(emu.cpu.pc);
        info!("CPU at{:04x}: {} {:02x} {:02x} {:02x} stat: A:{:02x} X:{:02x} Y:{:02x} FLAG:{:02x} S:{:02x}",
            emu.cpu.pc, OP_NAME[op as usize], op, emu.mem.load_u8(emu.cpu.pc+1),emu.mem.load_u8(emu.cpu.pc+2),
            emu.cpu.a, emu.cpu.x, emu.cpu.y, emu.cpu.sp.bits(), emu.cpu.s);
    }
    
    pub fn cpudebug_for_r(emu: &Emu) {
        let op = emu.mem.load_u8(emu.cpu.pc);
        debug!("{:04x} {} {:02x} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} CYC: {}",
        emu.cpu.pc, OP_NAME[op as usize], op, emu.cpu.a, emu.cpu.x, emu.cpu.y, emu.cpu.sp.bits(), emu.cpu.s, emu.stat.cycle_counter);
    }
    
    pub fn cpudebug_for_test(emu: &Emu) {
        let op = emu.mem.load_u8(emu.cpu.pc);
        debug!("{:04X} {:02X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        emu.cpu.pc, op, emu.cpu.a, emu.cpu.x, emu.cpu.y, emu.cpu.sp.bits(), emu.cpu.s);
    }

    pub fn run_cpu_with_callback(&mut self, inst: u32, callback: fn(&Emu)) {
        for _ in 0..inst {
            callback(self);
            self.run_cpu_once();
        }
    }

    pub fn run_cpu_once(&mut self) {
        let op = self.mem.load_u8(self.cpu.pc);
        let f = OP_FUNC[op as usize];
        let c = OP_CYCLE[op as usize];
        f(self);
        self.stat.cycle_counter += c as u32;
    }
    pub fn run_cpu_clocks(&mut self, clocks: u32) {
        let c = self.stat.cycle_counter;
        while self.stat.cycle_counter < c + clocks {
            self.run_cpu_once();
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x00,
            sp: Flags::from_bits_truncate(0x24),
            a: 0x00,
            x: 0x00,
            y: 0x00,
            s: 0xfd,
        }
    }
}
