struct Flags;
impl Flags {
    const N: u8 = 0b10000000;
    const V: u8 = 0b01000000;
    const B: u8 = 0b00110000;
    const D: u8 = 0b00001000;
    const I: u8 = 0b00000100;
    const Z: u8 = 0b00000010;
    const C: u8 = 0b00000001;
}

struct PpuRegisters;
impl PpuRegisters {
    const PPUCTRL: u16 = 0x2000;
    const PPUMASK: u16 = 0x2001;
    const PPUSTAT: u16 = 0x2002;
    const OAMADDR: u16 = 0x2003;
    const OAMDATA: u16 = 0x2004;
    const PPUSCRL: u16 = 0x2005;
    const PPUADDR: u16 = 0x2006;
    const PPUDATA: u16 = 0x2007;
    const OAMDMA : u16 = 0x4014;
}

pub struct Cpu {
    pc: u16,
    sp: u8,
    a:  u8,
    x:  u8,
    y:  u8,
    p:  u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x00,
            sp: 0xfd,
            a : 0x00,
            x : 0x00,
            y : 0x00,
            p : 0x36,
        }
    }
}