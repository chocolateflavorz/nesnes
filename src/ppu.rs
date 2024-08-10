use bitflags::bitflags;
use crate::cart::*;
use crate::mem::Mem;

pub struct Ppu {
    mem: Box<Mem>,
    palette: [u8; 32],
    addr: PpuAddressRegister,
    pub ctrl: ControlRegister,
    internal_data_buf: u8,
}

impl Ppu {
    pub fn new(mem: Box<Mem>) -> Self {
        Ppu {
            mem: mem,
            palette: [0; 32],
            addr: PpuAddressRegister::new(),
            ctrl: ControlRegister::new(),
            internal_data_buf: 0,
        }
    }

    fn write_to_address_register(&mut self, data: u8) {
        self.addr.update(data);
    }

    pub fn write_to_control_register(&mut self, val: u8) {
        self.ctrl.update(val);
    }
    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.mem.rom.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.mem.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),
            0x3f00..=0x3fff => self.palette[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mem.rom.screen_mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
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

    pub fn get(&self) -> u16 {
        ((self.value as u16) << 8) | (self.value as u16)
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
        self.insert(ControlRegister::from_bits_truncate(data));
    }
}
