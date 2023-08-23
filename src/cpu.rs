use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const N = 0b10000000;
        const V = 0b01000000;
        const B = 0b00110000;
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
    pub p: u8,
}

impl Cpu {
    pub fn nz_flags(&mut self, val: u8) {
        self.sp.set(Flags::N, val & 0x80 != 0);
        self.sp.set(Flags::Z, val == 0);
    }
    pub fn carry(&mut self, val: bool) {
        self.sp.set(Flags::C, val);
    }
    pub fn overflow(&mut self, val: bool) {
        self.sp.set(Flags::V, val);
    }
    pub fn interrupt(&mut self, val: bool) {
        self.sp.set(Flags::I, val);
    }

}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x00,
            sp: Flags::from_bits_truncate(0xfd),
            a: 0x00,
            x: 0x00,
            y: 0x00,
            p: 0x36,
        }
    }
}
