// undocumented ops
// https://www.nesdev.org/undocumented_opcodes.txt

#[inline]
pub fn  aac(emu: &mut Emu, val: u8) {
    emu.cpu.a = emu.cpu.a & val;
    emu.cpu.carry_flag(emu.cpu.a & 0x80 != 0);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  aax (emu: &mut Emu, addr: u16) {
    let r = emu.cpu.a & emu.cpu.x;
    emu.mem.store_u8(addr, r);
    emu.cpu.nz_flags(r);
}
#[inline]
pub fn arr (emu: &mut Emu, val: u8) {
    let r = (emu.cpu.a & val).rotate_right(1);
    let (bit_5, bit_6) = {
        (r & 0b0010_0000 != 0, r & 0b0100_0000 != 0)
    };
    match (bit_5, bit_6) {
        (true, true) => { emu.cpu.carry_flag(true); emu.cpu.overflow_flag(false); },
        (true, false) => { emu.cpu.carry_flag(false); emu.cpu.overflow_flag(true); },
        (false, true) => { emu.cpu.carry_flag(false); emu.cpu.overflow_flag(true); },
        (false, false) => { emu.cpu.carry_flag(false); emu.cpu.overflow_flag(false); }
    }
    emu.cpu.nz_flags(r);
}
#[inline]
pub fn asr (emu: &mut Emu, val: u8) {
    let r = (val & emu.cpu.a) >> 1;
    emu.cpu.carry_flag(emu.cpu.a & 0x01 != 0);
    emu.cpu.nz_flags(r);
}
#[inline]
pub fn atx (emu: &mut Emu, val: u8) {
    emu.cpu.x = emu.cpu.a & val;
    emu.cpu.nz_flags(emu.cpu.x);
}

// todo : write more
