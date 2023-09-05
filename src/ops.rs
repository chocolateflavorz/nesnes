#![cfg_attr(rustfmt, rustfmt_skip)]

use std::ops::Shl;
use crate::cpu::Flags;
use crate::emu::Emu;

// instruction(registers, emu.memory) -> additional cycles
pub type Behaviour = fn(&mut Emu) -> u8;

pub const OP_FUNC: [Behaviour; 0] = [];
pub const OP_CYCLE: [u8; 0] = [];
pub const OP_NAME: [&'static str; 0] = [];
pub const OP_LEN: [u8; 0] = [];

mod addressing {
    use crate::emu::Emu;
    #[inline] 
    pub fn immediate(emu: &Emu) -> u16 { 
        emu.cpu.pc 
    }
    #[inline] 
    pub fn zeropage(emu: &Emu) -> u16 { 
      emu.mem.load_u8(emu.cpu.pc) as u16 
    }
    #[inline]
    pub fn zeropage_x(emu: &Emu) -> u16 {
      emu.mem.load_u8(emu.cpu.pc).wrapping_add(emu.cpu.x) as u16 
    }
    #[inline]
    pub fn zeropage_y(emu: &Emu) -> u16 {
      emu.mem.load_u8(emu.cpu.pc).wrapping_add(emu.cpu.y) as u16 
    }
    #[inline] 
    pub fn absolute(emu: &Emu) -> u16 {
        emu.mem.load_u16(emu.cpu.pc) 
    }
    #[inline] 
    pub fn absolute_x(emu: &Emu) -> u16 {
        emu.mem.load_u16(emu.mem.load_u16(emu.cpu.pc)).wrapping_add(emu.cpu.x as u16) 
    }
    #[inline]
    pub fn absolute_y(emu: &Emu) -> u16 {
        emu.mem.load_u16(emu.mem.load_u16(emu.cpu.pc)).wrapping_add(emu.cpu.y as u16) 
    }
    #[inline]
    pub fn indirect_x(emu: &Emu) -> u16 {
        let addr = emu.mem.load_u8(emu.cpu.pc).wrapping_add(emu.cpu.x);
        emu.mem.load_u16(addr as u16)
    }
    #[inline]
    pub fn indirect_y(emu: &Emu) -> u16 {
        let addr = emu.mem.load_u8(emu.cpu.pc);
        emu.mem.load_u16(addr as u16).wrapping_add(emu.cpu.y as u16)
    }
} // mod addressing


#[inline]
fn brk(emu: &mut Emu) {
}
#[inline]
fn adc(emu: &mut Emu, val: u8) {
    let (r_a, v_a) = emu.cpu.a.overflowing_add(val);
    let (r_c, v_c) = r_a.overflowing_add(emu.cpu.sp.contains(Flags::C).into());
    emu.cpu.overflow_flag((r_c ^ emu.cpu.a) & 0x80 != 0);
    emu.cpu.a = r_c;
    emu.cpu.carry_flag(v_a || v_c);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline] 
fn and(emu: &mut Emu, val: u8) { 
    emu.cpu.a = emu.cpu.a & val;   
    emu.cpu.nz_flags(emu.cpu.a) 
}
#[inline]
fn asl(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    emu.cpu.carry_flag(val & 0x40 != 0);
    emu.mem.store_u8(addr, (val as i8).shl(1) as u8);
    emu.cpu.nz_flags(emu.cpu.a);
}
fn asl_i(emu: &mut Emu, val: u8) {
    emu.cpu.carry_flag(val & 0x40 != 0);
    emu.cpu.a = (val as i8).shl(1) as u8;
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
fn lda (emu: &mut Emu, val: u8) {
    emu.cpu.a = val;
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
fn ldx (emu: &mut Emu, val: u8) {
    emu.cpu.x = val;
    emu.cpu.nz_flags(emu.cpu.x);
}
#[inline]
fn ldy (emu: &mut Emu, val: u8) {
    emu.cpu.y = val;
    emu.cpu.nz_flags(emu.cpu.y);
}
#[inline]
fn sta (emu: &mut Emu, addr: u16) {
    emu.mem.store_u8(addr, emu.cpu.a);
}
#[inline]
fn stx (emu: &mut Emu, addr: u16) {
    emu.mem.store_u8(addr, emu.cpu.x);
}
#[inline]
fn sty (emu: &mut Emu, addr: u16) {
    emu.mem.store_u8(addr, emu.cpu.y);
}
#[inline]
fn bit (emu: &mut Emu, val: u8) {
    emu.cpu.overflow_flag(val & 0x40 != 0);
    emu.cpu.negative_flag(val & 0x80 != 0);
    emu.cpu.zero_flag(val & emu.cpu.a == 0);
}
#[inline]
fn cmp (emu: &mut Emu, val: u8) {
    let (r, v) = emu.cpu.a.overflowing_sub(val);
    emu.cpu.carry_flag(!v);
    emu.cpu.nz_flags(r);
}
#[inline]
fn cpx (emu: &mut Emu, val: u8) {
    let (r, v) = emu.cpu.x.overflowing_sub(val);
    emu.cpu.carry_flag(!v);
    emu.cpu.nz_flags(r);
}
#[inline]
fn cpy (emu: &mut Emu, val: u8) {
    let (r, v) = emu.cpu.y.overflowing_sub(val);
    emu.cpu.carry_flag(!v);
    emu.cpu.nz_flags(r);
}
#[inline]
fn dec (emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr).wrapping_sub(1);
    emu.mem.store_u8(addr, val);
    emu.cpu.nz_flags(val);
}
#[inline]
fn eor (emu: &mut Emu, val: u8) {
    emu.cpu.a = emu.cpu.a ^ val;
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
fn inc (emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr).wrapping_add(1);
    emu.mem.store_u8(addr, val);
    emu.cpu.nz_flags(val);
}
#[inline]
fn lsr (emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.mem.store_u8(addr, val >> 1);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
fn ora(emu: &mut Emu, val: u8) { 
    emu.cpu.a = emu.cpu.a | val;   
    emu.cpu.nz_flags(emu.cpu.a) 
}
#[inline]
fn rol(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(val & 0x80 != 0);
    emu.mem.store_u8(addr, (val << 1) | c);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
fn ror(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.mem.store_u8(addr, (val >> 1) | (c << 7));
    emu.cpu.nz_flags(emu.cpu.a);
}

fn tax_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.a;
    emu.cpu.nz_flags(emu.cpu.a);
}
fn tay_i (emu: &mut Emu) {
    emu.cpu.y = emu.cpu.a;
    emu.cpu.nz_flags(emu.cpu.a);
}
fn tsx_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.sp.bits();
    emu.cpu.nz_flags(emu.cpu.sp.bits());
}
fn txa_i (emu: &mut Emu) {
    emu.cpu.a = emu.cpu.x;
    emu.cpu.nz_flags(emu.cpu.x);
}
fn txs_i (emu: &mut Emu) {
    emu.cpu.sp = Flags::from_bits(emu.cpu.x).unwrap();
    emu.cpu.nz_flags(emu.cpu.x);
}
fn tya_i (emu: &mut Emu) {
    emu.cpu.a = emu.cpu.y;
    emu.cpu.nz_flags(emu.cpu.y);
}
fn dex_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.x.wrapping_sub(1);
    emu.cpu.nz_flags(emu.cpu.x);
}
fn dey_i (emu: &mut Emu) {
    emu.cpu.y = emu.cpu.y.wrapping_sub(1);
    emu.cpu.nz_flags(emu.cpu.y);
}
fn inx_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.x.wrapping_add(1);
    emu.cpu.nz_flags(emu.cpu.x);
}
fn iny_i (emu: &mut Emu) {
    emu.cpu.y = emu.cpu.y.wrapping_add(1);
    emu.cpu.nz_flags(emu.cpu.y);
}
fn lsr_i (emu: &mut Emu, val: u8) {
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.cpu.a = val >> 1;
    emu.cpu.nz_flags(emu.cpu.a);
}
fn rol_i (emu: &mut Emu) {
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(emu.cpu.a & 0x80 != 0);
    emu.cpu.a = (emu.cpu.a << 1) | c;
    emu.cpu.nz_flags(emu.cpu.a);
}
fn ror_i (emu: &mut Emu) {
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(emu.cpu.a & 0x01 != 0);
    emu.cpu.a = (emu.cpu.a >> 1) | (c << 7);
    emu.cpu.nz_flags(emu.cpu.a);
}





fn ora_ix(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2
}
fn ora_zp(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2
}
