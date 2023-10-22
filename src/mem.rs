const RAM_SIZE: usize = 0xffff;
const CPU_RAM_ZERO: u16 = 0x0000;
const CPU_RAM_ADDR_MASK: u16 = 0x07ff;
const CPU_RAM_MIRROR_END: u16 = 0x1fff;
const PPU_RAM_ZERO: u16 = 0x2000;
const PPU_RAM_ADDR_MASK: u16 = 0x2007;
const PPU_RAM_MIRROR_END: u16 = 0x3fff;
pub struct Mem {
    pub(crate) mem: [u8; RAM_SIZE],
}

impl Default for Mem {
    fn default() -> Self {
        Mem { mem: [0; RAM_SIZE] }
    }
}

impl Mem {
    #[inline]
    pub fn physical_address(addr: u16) -> u16 {
        match addr {
            CPU_RAM_ZERO..=CPU_RAM_MIRROR_END => addr & CPU_RAM_ADDR_MASK ,
            PPU_RAM_ZERO..=PPU_RAM_MIRROR_END => addr & PPU_RAM_ADDR_MASK ,
            _ => addr,
        }
    }
    #[inline]
    pub fn copy_from_slice(&mut self, offset: usize, slice: &[u8]) {
        self.mem[offset..offset + slice.len()].copy_from_slice(slice);
    }

    #[inline]
    pub fn store_u8(&mut self, addr: u16, data: u8) {
        let addr = Self::physical_address(addr);
        self.mem[addr as usize] = data;
    }
    #[inline]
    pub fn load_u8(&self, addr: u16) -> u8 {
        let addr = Self::physical_address(addr);
        self.mem[addr as usize]
    }
    #[inline]
    pub fn store_u16(&mut self, addr: u16, data: u16) {
        let addr = Self::physical_address(addr);
        let addr = addr as usize;
        self.mem[addr..=addr + 1].copy_from_slice(&data.to_le_bytes());
    }
    #[inline]
    pub fn load_u16(&self, addr: u16) -> u16 {
        let addr = Self::physical_address(addr);
        self.mem[addr as usize] as u16 | (self.mem[addr as usize + 1] as u16) << 8
    }
}
