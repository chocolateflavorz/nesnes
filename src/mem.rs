use crate::cart::Rom;

const RAM_SIZE: usize = 0xffff;
const CPU_RAM_ZERO: u16 = 0x0000;
const CPU_RAM_ADDR_MASK: u16 = 0x07ff;
const CPU_RAM_MIRROR_END: u16 = 0x1fff;
const PPU_RAM_ZERO: u16 = 0x2000;
const PPU_RAM_ADDR_MASK: u16 = 0x2007;
const PPU_RAM_MIRROR_END: u16 = 0x3fff;
const PRG_ROM_ZERO: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xffff;
pub struct Mem {
    pub mem: [u8; RAM_SIZE],
    rom: Rom
}

impl Default for Mem {
    fn default() -> Self {
        Mem { mem: [0; RAM_SIZE],
                rom: Rom::nothing()
        }
    }
}

impl Mem { 
    pub fn set_rom(&mut self, rom: Rom) {
        self.rom = rom;
    }
    #[inline]
    pub fn translate_address_w(&mut self, addr: u16) -> &mut [u8] {
        match addr {
            CPU_RAM_ZERO..=CPU_RAM_MIRROR_END => {
                &mut self.mem[addr as usize & CPU_RAM_ADDR_MASK as usize..] 
            },
            PPU_RAM_ZERO..=PPU_RAM_MIRROR_END => {
                &mut self.mem[addr as usize  & PPU_RAM_ADDR_MASK as usize..]
            },
            PRG_ROM_ZERO..=PRG_ROM_END => {
                let mut addr = addr - PRG_ROM_ZERO;
                if self.rom.prg_rom.len() == 0x4000 && (addr & 0x4000 == 0x4000) {
                    addr = addr ^ 0x4000;
                }
                &mut self.rom.prg_rom[addr as usize..]
            },
            _ => {
                panic!("illegal address translation occured at: {:04x}", addr)
            },
        }
    }

    #[inline]
    pub fn translate_address_r(&self, addr: u16) -> &[u8] {
        match addr {
            CPU_RAM_ZERO..=CPU_RAM_MIRROR_END => {
                &self.mem[addr as usize & CPU_RAM_ADDR_MASK as usize..] 
            },
            PPU_RAM_ZERO..=PPU_RAM_MIRROR_END => {
                &self.mem[addr as usize  & PPU_RAM_ADDR_MASK as usize..]
            },
            PRG_ROM_ZERO..=PRG_ROM_END => {
                let mut addr = addr - PRG_ROM_ZERO;
                if self.rom.prg_rom.len() == 0x4000 && (addr & 0x4000 == 0x4000) {
                    addr = addr ^ 0x4000;
                }
                &self.rom.prg_rom[addr as usize..]
            },
            _ => {
                panic!("illegal address translation occured at: {:04x}", addr)
            },
        }
    }
    #[inline]
    pub fn copy_from_slice(&mut self, offset: usize, slice: &[u8]) {
        self.mem[offset..offset + slice.len()].copy_from_slice(slice);
    }

    #[inline]
    pub fn store_u8(&mut self, addr: u16, data: u8) {
        let p = Self::translate_address_w(self, addr);
        p[0] = data;
    }
    #[inline]
    pub fn load_u8(&self, addr: u16) -> u8 {
        let p = Self::translate_address_r(self, addr);
        p[0]
    }
    #[inline]
    pub fn store_u16(&mut self, addr: u16, data: u16) {
        let p = Self::translate_address_w(self, addr);
        p[0..=1].copy_from_slice(&data.to_le_bytes());
    }
    #[inline]
    pub fn load_u16(&self, addr: u16) -> u16 {
        let p = Self::translate_address_r(self, addr);
        p[0] as u16 | ((p[1] as u16) << 8)
    }
}
