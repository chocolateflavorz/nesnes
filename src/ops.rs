#![cfg_attr(rustfmt, rustfmt_skip)]

use std::ops::Shl;
use crate::cpu::Flags;
use crate::emu::Emu;

pub type Behaviour = fn(&mut Emu);

pub const OP_FUNC: [Behaviour; 0] = [];
pub const OP_CYCLE: [u8; 0] = [];
pub const OP_NAME: [&'static str; 0] = [];


mod addressing {
    use crate::emu::Emu;
    #[inline] 
    pub fn immediate (emu: &Emu) -> u16{ 
        emu.cpu.pc + 1
    }
    #[inline] 
    pub fn zeropage(emu: &Emu) -> u16 { 
      emu.mem.load_u8(emu.cpu.pc+1) as u16 
    }
    #[inline]
    pub fn zeropage_x(emu: &Emu) -> u16 {
      emu.mem.load_u8(emu.cpu.pc+1).wrapping_add(emu.cpu.x) as u16 
    }
    #[inline]
    pub fn zeropage_y(emu: &Emu) -> u16 {
      emu.mem.load_u8(emu.cpu.pc+1).wrapping_add(emu.cpu.y) as u16 
    }
    #[inline] 
    pub fn absolute(emu: &Emu) -> u16 {
        emu.mem.load_u16(emu.cpu.pc+1) 
    }
    #[inline] 
    pub fn absolute_x(emu: &Emu) -> u16 {
        emu.mem.load_u16(emu.mem.load_u16(emu.cpu.pc+1)).wrapping_add(emu.cpu.x as u16) 
    }
    #[inline]
    pub fn absolute_y(emu: &Emu) -> u16 {
        emu.mem.load_u16(emu.mem.load_u16(emu.cpu.pc+1)).wrapping_add(emu.cpu.y as u16) 
    }
    #[inline]
    pub fn indirect_x(emu: &Emu) -> u16 {
        let addr = emu.mem.load_u8(emu.cpu.pc+1).wrapping_add(emu.cpu.x);
        emu.mem.load_u16(addr as u16)
    }
    #[inline]
    pub fn indirect_y(emu: &Emu) -> u16 {
        let addr = emu.mem.load_u8(emu.cpu.pc+1);
        emu.mem.load_u16(addr as u16).wrapping_add(emu.cpu.y as u16)
    }
    #[inline]
    pub fn relative(emu: &Emu) -> u16 {
        emu.cpu.pc + 2 + emu.mem.load_u8(emu.cpu.pc+1) as u16
    }
} // mod addressing

mod stack {
use crate::emu::Emu;
#[inline]
pub fn push_u8 (emu: &mut Emu, val: u8) {
    emu.mem.store_u8(emu.cpu.stack_ptr(), val);
    emu.cpu.s -= 1;
}
#[inline]
pub fn pop_u8 (emu: &mut Emu) -> u8 {
    emu.cpu.s += 1;
    emu.mem.load_u8(emu.cpu.stack_ptr())
}
#[inline]
pub fn push_u16 (emu: &mut Emu, val: u16) {
    emu.mem.store_u16(emu.cpu.stack_ptr(), val);
    emu.cpu.s -= 2;
}
pub fn pop_u16 (emu: &mut Emu) -> u16 {
    emu.cpu.s += 2;
    emu.mem.load_u16(emu.cpu.stack_ptr())
}
} // mod stack

#[inline]
fn branch(emu: &mut Emu, cond: bool) {
        let pc = emu.cpu.pc + 2;
        if cond {
            emu.cpu.pc = addressing::relative(emu);
            emu.stat.cycle_counter += ((emu.cpu.pc ^ pc) & 0xFF00 != 0)  as u32;
        } else {
            emu.cpu.pc = pc;
        }
}

pub fn  undefined(emu: &mut Emu) {
}
pub fn  brk_i (emu: &mut Emu) {
    stack::push_u16(emu, emu.cpu.pc + 1);
    stack::push_u8(emu, emu.cpu.sp.bits());
    emu.cpu.interrupt_flag(true);
    emu.cpu.pc = emu.mem.load_u16(0xFFFE);
}
pub fn nop_i (emu: &mut Emu) {
    emu.cpu.pc += 1;
}

#[inline]
pub fn  adc(emu: &mut Emu, val: u8) {
    let (r_a, v_a) = emu.cpu.a.overflowing_add(val);
    let (r_c, v_c) = r_a.overflowing_add(emu.cpu.sp.contains(Flags::C).into());
    emu.cpu.overflow_flag((r_c ^ emu.cpu.a) & 0x80 != 0);
    emu.cpu.a = r_c;
    emu.cpu.carry_flag(v_a || v_c);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline] 
pub fn  and(emu: &mut Emu, val: u8) { 
    emu.cpu.a = emu.cpu.a & val;   
    emu.cpu.nz_flags(emu.cpu.a) 
}
#[inline]
pub fn  asl(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    emu.cpu.carry_flag(val & 0x40 != 0);
    emu.mem.store_u8(addr, (val as i8).shl(1) as u8);
    emu.cpu.nz_flags(emu.cpu.a);
}
pub fn  asl_i(emu: &mut Emu, val: u8) {
    emu.cpu.carry_flag(val & 0x40 != 0);
    emu.cpu.a = (val as i8).shl(1) as u8;
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  lda (emu: &mut Emu, val: u8) {
    emu.cpu.a = val;
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  ldx (emu: &mut Emu, val: u8) {
    emu.cpu.x = val;
    emu.cpu.nz_flags(emu.cpu.x);
}
#[inline]
pub fn  ldy (emu: &mut Emu, val: u8) {
    emu.cpu.y = val;
    emu.cpu.nz_flags(emu.cpu.y);
}
#[inline]
pub fn  sta (emu: &mut Emu, addr: u16) {
    emu.mem.store_u8(addr, emu.cpu.a);
}
#[inline]
pub fn  stx (emu: &mut Emu, addr: u16) {
    emu.mem.store_u8(addr, emu.cpu.x);
}
#[inline]
pub fn  sty (emu: &mut Emu, addr: u16) {
    emu.mem.store_u8(addr, emu.cpu.y);
}
#[inline]
pub fn  bit (emu: &mut Emu, val: u8) {
    emu.cpu.overflow_flag(val & 0x40 != 0);
    emu.cpu.negative_flag(val & 0x80 != 0);
    emu.cpu.zero_flag(val & emu.cpu.a == 0);
}
#[inline]
pub fn  cmp (emu: &mut Emu, val: u8) {
    let (r, v) = emu.cpu.a.overflowing_sub(val);
    emu.cpu.carry_flag(!v);
    emu.cpu.nz_flags(r);
}
#[inline]
pub fn  cpx (emu: &mut Emu, val: u8) {
    let (r, v) = emu.cpu.x.overflowing_sub(val);
    emu.cpu.carry_flag(!v);
    emu.cpu.nz_flags(r);
}
#[inline]
pub fn  cpy (emu: &mut Emu, val: u8) {
    let (r, v) = emu.cpu.y.overflowing_sub(val);
    emu.cpu.carry_flag(!v);
    emu.cpu.nz_flags(r);
}
#[inline]
pub fn  dec (emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr).wrapping_sub(1);
    emu.mem.store_u8(addr, val);
    emu.cpu.nz_flags(val);
}
#[inline]
pub fn  eor (emu: &mut Emu, val: u8) {
    emu.cpu.a = emu.cpu.a ^ val;
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  inc (emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr).wrapping_add(1);
    emu.mem.store_u8(addr, val);
    emu.cpu.nz_flags(val);
}
#[inline]
pub fn  lsr (emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.mem.store_u8(addr, val >> 1);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  ora(emu: &mut Emu, val: u8) { 
    emu.cpu.a = emu.cpu.a | val;   
    emu.cpu.nz_flags(emu.cpu.a) 
}
#[inline]
pub fn  rol(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(val & 0x80 != 0);
    emu.mem.store_u8(addr, (val << 1) | c);
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  ror(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.mem.store_u8(addr, (val >> 1) | (c << 7));
    emu.cpu.nz_flags(emu.cpu.a);
}
#[inline]
pub fn  sbc(emu: &mut Emu, val: u8) {
    let (r_a, v_a) = emu.cpu.a.overflowing_sub(val);
    let (r_c, v_c) = r_a.overflowing_sub((!emu.cpu.sp.contains(Flags::C)).into());
    emu.cpu.overflow_flag((r_c ^ emu.cpu.a) & 0x80 != 0);
    emu.cpu.a = r_c;
    emu.cpu.carry_flag(v_a || v_c);
    emu.cpu.nz_flags(emu.cpu.a);
}

pub fn  tax_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.a;
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  tay_i (emu: &mut Emu) {
    emu.cpu.y = emu.cpu.a;
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  tsx_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.sp.bits();
    emu.cpu.nz_flags(emu.cpu.sp.bits());
    emu.cpu.pc += 1;
}
pub fn  txa_i (emu: &mut Emu) {
    emu.cpu.a = emu.cpu.x;
    emu.cpu.nz_flags(emu.cpu.x);
    emu.cpu.pc += 1;
}
pub fn  txs_i (emu: &mut Emu) {
    emu.cpu.sp = Flags::from_bits(emu.cpu.x).unwrap();
    emu.cpu.nz_flags(emu.cpu.x);
    emu.cpu.pc += 1;
}
pub fn  tya_i (emu: &mut Emu) {
    emu.cpu.a = emu.cpu.y;
    emu.cpu.nz_flags(emu.cpu.y);
    emu.cpu.pc += 1;
}
pub fn  dex_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.x.wrapping_sub(1);
    emu.cpu.nz_flags(emu.cpu.x);
    emu.cpu.pc += 1;
}
pub fn  dey_i (emu: &mut Emu) {
    emu.cpu.y = emu.cpu.y.wrapping_sub(1);
    emu.cpu.nz_flags(emu.cpu.y);
    emu.cpu.pc += 1;
}
pub fn  inx_i (emu: &mut Emu) {
    emu.cpu.x = emu.cpu.x.wrapping_add(1);
    emu.cpu.nz_flags(emu.cpu.x);
    emu.cpu.pc += 1;
}
pub fn  iny_i (emu: &mut Emu) {
    emu.cpu.y = emu.cpu.y.wrapping_add(1);
    emu.cpu.nz_flags(emu.cpu.y);
    emu.cpu.pc += 1;
}
pub fn  lsr_i (emu: &mut Emu, val: u8) {
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.cpu.a = val >> 1;
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  rol_i (emu: &mut Emu) {
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(emu.cpu.a & 0x80 != 0);
    emu.cpu.a = (emu.cpu.a << 1) | c;
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  ror_i (emu: &mut Emu) {
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(emu.cpu.a & 0x01 != 0);
    emu.cpu.a = (emu.cpu.a >> 1) | (c << 7);
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  pha_i (emu: &mut Emu) {
    stack::push_u8(emu, emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  php_i (emu: &mut Emu) {
    stack::push_u8(emu, emu.cpu.sp.bits());
    emu.cpu.pc += 1;
}
pub fn  pla_i (emu: &mut Emu) {
    emu.cpu.a = stack::pop_u8(emu);
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  plp_i (emu: &mut Emu) {
    emu.cpu.sp = Flags::from_bits(stack::pop_u8(emu)).unwrap();
    emu.cpu.pc += 1;
}
pub fn  jmp_abs(emu: &mut Emu) {
    emu.cpu.pc = addressing::absolute(emu);
}
pub fn  jmp_ind(emu: &mut Emu) {
    emu.cpu.pc = emu.mem.load_u16(emu.mem.load_u16(emu.cpu.pc+1));
}
pub fn  jsr_abs(emu: &mut Emu) {
    let ret = emu.cpu.pc + 2;
    stack::push_u16(emu, ret);
    emu.cpu.pc = addressing::absolute(emu);
}
pub fn  rts_i(emu: &mut Emu) {
    emu.cpu.pc = stack::pop_u16(emu) + 1;
}
pub fn  rti_i(emu: &mut Emu) {
    emu.cpu.sp = Flags::from_bits(stack::pop_u8(emu)).unwrap();
    emu.cpu.pc = stack::pop_u16(emu);
}
pub fn  bcc_rel(emu: &mut Emu) {
    branch(emu, !emu.cpu.sp.contains(Flags::C));
} 
pub fn bcs_rel(emu: &mut Emu) {
    branch(emu, emu.cpu.sp.contains(Flags::C));
}
pub fn beq_rel(emu: &mut Emu) {
    branch(emu, emu.cpu.sp.contains(Flags::Z));
}
pub fn bmi_rel(emu: &mut Emu) {
    branch(emu, emu.cpu.sp.contains(Flags::N));
}
pub fn bne_rel(emu: &mut Emu) {
    branch(emu, !emu.cpu.sp.contains(Flags::Z));
}
pub fn bpl_rel(emu: &mut Emu) {
    branch(emu, !emu.cpu.sp.contains(Flags::N));
}
pub fn bvc_rel(emu: &mut Emu) {
    branch(emu, !emu.cpu.sp.contains(Flags::V));
}
pub fn bvs_rel(emu: &mut Emu) {
    branch(emu, emu.cpu.sp.contains(Flags::V));
}
pub fn clc_i(emu: &mut Emu) {
    emu.cpu.carry_flag(false);
    emu.cpu.pc += 1;
}
pub fn cli_i(emu: &mut Emu) {
    emu.cpu.interrupt_flag(false);
    emu.cpu.pc += 1;
}
pub fn clv_i(emu: &mut Emu) {
    emu.cpu.overflow_flag(false);
    emu.cpu.pc += 1;
}
pub fn sec_i(emu: &mut Emu) {
    emu.cpu.carry_flag(true);
    emu.cpu.pc += 1;
}
pub fn sei_i(emu: &mut Emu) {
    emu.cpu.interrupt_flag(true);
    emu.cpu.pc += 1;
}

pub fn ora_inx(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::indirect_x(emu)));
    emu.cpu.pc += 2;
}
pub fn ora_zpg(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
