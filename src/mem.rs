const MEM_SIZE: usize = 0xffff;
pub struct Mem {
    mem: [u8; MEM_SIZE],
}

impl Default for Mem {
    fn default() -> Self {
        Mem { mem: [0; MEM_SIZE] }
    }
}

impl Mem {
    #[inline]
    pub fn copy_from_slice(&mut self, offset: usize, slice: &[u8]) {
        self.mem[offset..offset + slice.len()].copy_from_slice(slice);
    }

    #[inline]
    pub fn store_u8(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
    #[inline]
    pub fn load_u8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    #[inline]
    pub fn store_u16(&mut self, addr: u16, data: u16) {
        let addr = addr as usize;
        self.mem[addr..addr + 1].copy_from_slice(&data.to_le_bytes());
    }
    #[inline]
    pub fn load_u16(&self, addr: u16) -> u16 {
        (self.mem[addr as usize] as u16) << 8 | self.mem[addr as usize + 1] as u16
    }
}
