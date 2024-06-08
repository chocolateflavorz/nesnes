use bitflags::bitflags;
use log::debug;
use log::info;

use crate::cart::Mirroring::*;
use crate::cart::*;
use crate::emu::Emu;

pub struct Ppu {
    rom: Box<Rom>,
    vram: [u8; 0x800],
    oam: [u8; 0x100],
    palette: [u8; 32],
    address_register: PpuAddressRegister,
}

impl Ppu {
    fn new(rom: Box<Rom>) -> Self {
        Ppu {
            rom: rom,
            vram: [0; 0x800],
            oam: [0; 0x100],
            palette: [0; 32],
        }
    }

    fn write_to_address_register(&mut self, data: u8) {
        self.address_register.update(data);
    }
}

pub struct PpuAddressRegister {
    pub value: u16,
    hi_ptr: bool,
}

impl PpuAddressRegister {
    pub fn new() -> Self {
        PpuAddressRegister {
            value: 0,
            hi_ptr: true,
        }
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value |= (data as u16) << 8;
        } else {
            self.value |= data as u16;
        }
        self.value &= 0x3fff;
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        self.value += inc as u16;
        self.value &= 0x3fff;
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}

bitflags! {
    pub struct ControlRegister: u8 {
       const NAMETABLE1              = 0b00000001;
       const NAMETABLE2              = 0b00000010;
       const VRAM_ADD_INCREMENT      = 0b00000100;
       const SPRITE_PATTERN_ADDR     = 0b00001000;
       const BACKROUND_PATTERN_ADDR  = 0b00010000;
       const SPRITE_SIZE             = 0b00100000;
       const MASTER_SLAVE_SELECT     = 0b01000000;
       const GENERATE_NMI            = 0b10000000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0b00000000)
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}
