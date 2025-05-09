#![cfg_attr(rustfmt, rustfmt_skip)]

use std::ops::{Shl, Not};
use crate::cpu::Flags;
use crate::emu::Emu;

pub type Behaviour = fn(&mut Emu);

#[rustfmt::skip]
pub const OP_FUNC: [Behaviour; 256] = 
//        0       1      2         3      4        5       6         7      8     9      A        B      C        D       E         F
[/*0*/ brk,     ora_inx, undef,   undef, undef,   ora_zpg, asl_zpg, undef, php_i, ora_imm, asl_i, undef, undef,   ora_abs, asl_abs, undef,
 /*1*/ bpl_rel, ora_iny, undef,   undef, undef,   ora_zpx, asl_zpx, undef, clc_i, ora_aby, undef, undef, undef,   ora_abx, asl_abx, undef,
 /*2*/ jsr_abs, and_inx, undef,   undef, bit_zpg, and_zpg, rol_zpg, undef, plp_i, and_imm, rol_i, undef, bit_abs, and_abs, rol_abs, undef,
 /*3*/ bmi_rel, and_iny, undef,   undef, undef,   and_zpx, rol_zpx, undef, sec_i, and_aby, undef, undef, undef,   and_abx, rol_abx, undef,
 /*4*/ rti_i,   eor_inx, undef,   undef, undef,   eor_zpg, lsr_zpg, undef, pha_i, eor_imm, lsr_i, undef, jmp_abs, eor_abs, lsr_abs, undef,
 /*5*/ bvc_rel, eor_iny, undef,   undef, undef,   eor_zpx, lsr_zpx, undef, cli_i, eor_aby, undef, undef, undef,   eor_abx, lsr_abx, undef,
 /*6*/ rts_i,   adc_inx, undef,   undef, undef,   adc_zpg, ror_zpg, undef, pla_i, adc_imm, ror_i, undef, jmp_ind, adc_abs, ror_abs, undef,
 /*7*/ bvs_rel, adc_iny, undef,   undef, undef,   adc_zpx, ror_zpx, undef, sei_i, adc_aby, undef, undef, undef,   adc_abx, ror_abx, undef,
 /*8*/ undef,   sta_inx, undef,   undef, sty_zpg, sta_zpg, stx_zpg, undef, dey_i, undef,   txa_i, undef, sty_abs, sta_abs, stx_abs, undef,
 /*9*/ bcc_rel, sta_iny, undef,   undef, sty_zpx, sta_zpx, stx_zpy, undef, tya_i, sta_aby, txs_i, undef, undef,   sta_abx, undef,   undef,
 /*A*/ ldy_imm, lda_inx, ldx_imm, undef, ldy_zpg, lda_zpg, ldx_zpg, undef, tay_i, lda_imm, tax_i, undef, ldy_abs, lda_abs, ldx_abs, undef,
 /*B*/ bcs_rel, lda_iny, undef,   undef, ldy_zpx, lda_zpx, ldx_zpy, undef, clv_i, lda_aby, tsx_i, undef, ldy_abx, lda_abx, ldx_aby, undef,
 /*C*/ cpy_imm, cmp_inx, undef,   undef, cpy_zpg, cmp_zpg, dec_zpg, undef, iny_i, cmp_imm, dex_i, undef, cpy_abs, cmp_abs, dec_abs, undef,
 /*D*/ bne_rel, cmp_iny, undef,   undef, undef,   cmp_zpx, dec_zpx, undef, cld_i, cmp_aby, undef, undef, undef,   cmp_abx, dec_abx, undef,
 /*E*/ cpx_imm, sbc_inx, undef,   undef, cpx_zpg, sbc_zpg, inc_zpg, undef, inx_i, sbc_imm, nop,   undef, cpx_abs, sbc_abs, inc_abs, undef,
 /*F*/ beq_rel, sbc_iny, undef,   undef, undef,   sbc_zpx, inc_zpx, undef, sed_i, sbc_aby, undef, undef, undef,   sbc_abx, inc_abx, undef
];
#[rustfmt::skip]
pub const OP_CYCLE: [u8; 256] = [
2, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 0, 4, 6, 0,
2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
6, 6, 0, 0, 3, 3, 5, 0, 4, 2, 2, 0, 4, 4, 6, 0,
2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
6, 6, 0, 0, 0, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0,
2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
6, 6, 0, 0, 0, 3, 5, 0, 2, 2, 2, 0, 3, 4, 6, 0,
2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
6, 6, 0, 0, 3, 3, 3, 0, 2, 2, 2, 0, 3, 4, 4, 0,
2, 6, 0, 0, 4, 4, 4, 0, 2, 5, 2, 0, 4, 4, 4, 0,
2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 3, 4, 4, 0,
2, 5, 0, 0, 4, 4, 6, 0, 2, 4, 2, 0, 4, 4, 4, 0,
2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 3, 4, 4, 0,
2, 5, 0, 0, 4, 4, 6, 0, 0, 4, 2, 0, 4, 4, 4, 0,
2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 3, 4, 4, 0,
2, 5, 0, 0, 4, 4, 6, 0, 0, 4, 2, 0, 4, 4, 4, 0
];
pub const OP_NAME: [&'static str; 256] = [
"brk", "ora", "undef", "undef", "undef", "ora", "asl", "undef", "php", "ora", "asl", "undef", "undef", "ora", "asl", "undef",
"bpl", "ora", "undef", "undef", "undef", "ora", "asl", "undef", "clc", "ora", "undef", "undef", "undef", "ora", "asl", "undef",
"jsr", "and", "undef", "undef", "bit", "and", "rol", "undef", "plp", "and", "rol", "undef", "bit", "and", "rol", "undef",
"bmi", "and", "undef", "undef", "undef", "and", "rol", "undef", "sec", "and", "undef", "undef", "undef", "and", "rol", "undef",
"rti", "eor", "undef", "undef", "undef", "eor", "lsr", "undef", "pha", "eor", "lsr", "undef", "jmp", "eor", "lsr", "undef",
"bvc", "eor", "undef", "undef", "undef", "eor", "lsr", "undef", "cli", "eor", "undef", "undef", "undef", "eor", "lsr", "undef",
"rts", "adc", "undef", "undef", "undef", "adc", "ror", "undef", "pla", "adc", "ror", "undef", "jmp", "adc", "ror", "undef",
"bvs", "adc", "undef", "undef", "undef", "adc", "ror", "undef", "sei", "adc", "undef", "undef", "undef", "adc", "ror", "undef",
"undef", "sta", "undef", "undef", "sty", "sta", "stx", "undef", "dey", "undef", "txa", "undef", "sty", "sta", "stx", "undef",
"bcc", "sta", "undef", "undef", "sty", "sta", "stx", "undef", "tya", "sta", "txs", "undef", "undef", "sta", "undef", "undef",
"ldy", "lda", "ldx", "undef", "ldy", "lda", "ldx", "undef", "tay", "lda", "tax", "undef", "ldy", "lda", "ldx", "undef",
"bcs", "lda", "undef", "undef", "ldy", "lda", "ldx", "undef", "clv", "lda", "tsx", "undef", "ldy", "lda", "ldx", "undef",
"cpy", "cmp", "undef", "undef", "cpy", "cmp", "dec", "undef", "iny", "cmp", "dex", "undef", "cpy", "cmp", "dec", "undef",
"bne", "cmp", "undef", "undef", "undef", "cmp", "dec", "undef", "cld", "cmp", "undef", "undef", "undef", "cmp", "dec", "undef",
"cpx", "sbc", "undef", "undef", "cpx", "sbc", "inc", "undef", "inx", "sbc", "nop", "undef", "cpx", "sbc", "inc", "undef",
"beq", "sbc", "undef", "undef", "undef", "sbc", "inc", "undef", "sed", "sbc", "undef", "undef", "undef", "sbc", "inc", "undef"
];



mod addressing {
    use log::debug;

    use crate::emu::Emu;
    #[inline] 
    pub fn immediate (emu: &Emu) -> u16{ 
        let r =emu.cpu.pc + 1;
        debug!("#${:02x}", emu.mem.load_u8(r));
        r
    }
    #[inline] 
    pub fn zeropage(emu: &Emu) -> u16 { 
      let r = emu.mem.load_u8(emu.cpu.pc+1) as u16;
      debug!("${:02x} = {:02x}", r, emu.mem.load_u8(r));
        r
    }
    #[inline]
    pub fn zeropage_x(emu: &Emu) -> u16 {
      let r = emu.mem.load_u8(emu.cpu.pc+1).wrapping_add(emu.cpu.x) as u16;
        debug!("${:02x},X @ {:02x} = {:02x}", emu.mem.load_u8(emu.cpu.pc+1), r, emu.mem.load_u8(r));
        r
    }
    #[inline]
    pub fn zeropage_y(emu: &Emu) -> u16 {
      let r = emu.mem.load_u8(emu.cpu.pc+1).wrapping_add(emu.cpu.y) as u16 ;
        debug!("${:02x},Y @ {:02x} = {:02x}", emu.mem.load_u8(emu.cpu.pc+1), r, emu.mem.load_u8(r));
        r
    }
    #[inline] 
    pub fn absolute(emu: &Emu) -> u16 {
        let r = emu.mem.load_u16(emu.cpu.pc+1) ;
        debug!("abs ${:04x} = {:04x} = {:04x}", emu.cpu.pc+1, r, emu.mem.load_u16(r));
        r
    }
    #[inline] 
    pub fn absolute_x(emu: &Emu) -> u16 {
        let r =  emu.mem.load_u16(emu.cpu.pc+1).wrapping_add(emu.cpu.x as u16) ;
        debug!("${:04x},X @ {:04x} = {:02x}", emu.mem.load_u16(emu.cpu.pc+1), r, emu.mem.load_u8(r));
        r
    }
    #[inline]
    pub fn absolute_y(emu: &Emu) -> u16 {
        let r = emu.mem.load_u16(emu.cpu.pc+1).wrapping_add(emu.cpu.y as u16);
        debug!("${:04x},Y @ {:04x} = {:02x}", emu.mem.load_u16(emu.cpu.pc+1), r, emu.mem.load_u8(r));
        r
    }
    #[inline]
    pub fn indirect_x(emu: &Emu) -> u16 {
        let addr = emu.mem.load_u8(emu.cpu.pc+1).wrapping_add(emu.cpu.x);
        let r = emu.mem.load_u8(addr as u16) as u16 | ((emu.mem.load_u8(addr.wrapping_add(1) as u16) as u16) << 8) as u16;
        debug!("(${:02x},X) @ {:02x} = {:04x} = {:02x}", emu.mem.load_u8(emu.cpu.pc+1), addr, r, emu.mem.load_u8(r));
        r
    }
    #[inline]
    pub fn indirect_y(emu: &Emu) -> u16 {
        let addr = emu.mem.load_u8(emu.cpu.pc+1);
        let r = emu.mem.load_u8(addr as u16) as u16 | ((emu.mem.load_u8(addr.wrapping_add(1) as u16) as u16) << 8) as u16;
        let r = r.wrapping_add(emu.cpu.y as u16);
        debug!("(${:02x}),Y = {:04x} @ {:04x} = {:02x}", emu.mem.load_u8(emu.cpu.pc+1), emu.mem.load_u16(addr as u16), r, emu.mem.load_u8(r));
        r
    }
    #[inline]
    pub fn relative(emu: &Emu) -> u16 {
        let r = (emu.cpu.pc as i16 + 2 + ((emu.mem.load_u8(emu.cpu.pc+1) as i8) as i16)) as u16;
        debug!("${:04x}", r);
        r
    }
} // mod addressing

mod stack {
use crate::emu::Emu;
#[inline]
pub fn push_u8 (emu: &mut Emu, val: u8) {
    emu.mem.store_u8(emu.cpu.stack_ptr(), val);
    emu.cpu.s = emu.cpu.s.wrapping_sub(1);
}
#[inline]
pub fn pop_u8 (emu: &mut Emu) -> u8 {
    emu.cpu.s = emu.cpu.s.wrapping_add(1);
    emu.mem.load_u8(emu.cpu.stack_ptr())
}
#[inline]
pub fn push_u16 (emu: &mut Emu, val: u16) {
    push_u8(emu, (val >> 8) as u8);
    push_u8(emu, (val & 0xFF) as u8);
}
pub fn pop_u16 (emu: &mut Emu) -> u16 {
    let l = pop_u8(emu) as u16;
    let h = pop_u8(emu) as u16;
    h << 8 | l
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

pub fn  undef(emu: &mut Emu) {
    println!("undefined opcode {:04X} {:02X}", emu.cpu.pc, emu.mem.load_u8(emu.cpu.pc) );
    emu.cpu.pc += 1;
}
pub fn  brk (emu: &mut Emu) {
    stack::push_u16(emu, emu.cpu.pc + 1);
    stack::push_u8(emu, emu.cpu.sp.bits());
    emu.cpu.interrupt_flag(true);
    emu.cpu.pc = emu.mem.load_u16(0xFFFE);
}
pub fn nop (emu: &mut Emu) {
    emu.cpu.pc += 1;
}

#[inline]
pub fn  adc(emu: &mut Emu, val: u8) {
    let (r_a, v_a) = emu.cpu.a.overflowing_add(val);
    let (r_c, v_c) = r_a.overflowing_add(emu.cpu.sp.contains(Flags::C).into());
    emu.cpu.overflow_flag((val ^ r_c) & (r_c ^ emu.cpu.a) & 0x80 != 0);
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
    emu.cpu.carry_flag(val & 0x80 != 0);
    emu.mem.store_u8(addr, val << 1);
    emu.cpu.nz_flags(val << 1);
}
pub fn  asl_i(emu: &mut Emu) {
    let val = emu.cpu.a;
    emu.cpu.carry_flag(val & 0x80 != 0);
    emu.cpu.a = val << 1;
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
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
    emu.cpu.nz_flags(val >> 1);
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
    emu.cpu.nz_flags((val << 1) | c);
}
#[inline]
pub fn  ror(emu: &mut Emu, addr: u16) {
    let val = emu.mem.load_u8(addr);
    let c = emu.cpu.sp.contains(Flags::C) as u8;
    emu.cpu.carry_flag(val & 0x01 != 0);
    emu.mem.store_u8(addr, (val >> 1) | (c << 7));
    emu.cpu.nz_flags((val >> 1) | (c << 7));
}
#[inline]
pub fn  sbc(emu: &mut Emu, val: u8) {
    let (r_a, v_a) = emu.cpu.a.overflowing_add(val.not());
    let (r_c, v_c) = r_a.overflowing_add(emu.cpu.sp.contains(Flags::C).into());
    emu.cpu.overflow_flag((r_c ^ !val) & (r_c ^ emu.cpu.a) & 0x80 != 0);
    emu.cpu.a = r_c;
    emu.cpu.carry_flag(v_a || v_c);
    emu.cpu.nz_flags(emu.cpu.a);
}

// ops with addressing mode
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
    emu.cpu.x = emu.cpu.s;
    emu.cpu.nz_flags(emu.cpu.s);
    emu.cpu.pc += 1;
}
pub fn  txa_i (emu: &mut Emu) {
    emu.cpu.a = emu.cpu.x;
    emu.cpu.nz_flags(emu.cpu.x);
    emu.cpu.pc += 1;
}
pub fn  txs_i (emu: &mut Emu) {
    emu.cpu.s = emu.cpu.x;
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
pub fn  lsr_i (emu: &mut Emu) {
    let val = emu.cpu.a;
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
    stack::push_u8(emu, emu.cpu.sp.bits() | Flags::A.bits() | Flags::B.bits());
    emu.cpu.pc += 1;
}
pub fn  pla_i (emu: &mut Emu) {
    emu.cpu.a = stack::pop_u8(emu);
    emu.cpu.nz_flags(emu.cpu.a);
    emu.cpu.pc += 1;
}
pub fn  plp_i (emu: &mut Emu) {
    emu.cpu.sp = Flags::from_bits((stack::pop_u8(emu) | Flags::A.bits()) & Flags::B.bits().not()).unwrap();
    emu.cpu.pc += 1;
}
pub fn  jmp_abs(emu: &mut Emu) {
    emu.cpu.pc = addressing::absolute(emu);
}
pub fn  jmp_ind(emu: &mut Emu) {
    let r = addressing::absolute(emu);
    if r & 0x00ff == 0x00ff {
        let l = emu.mem.load_u8(r);
        let h = emu.mem.load_u8(r & 0xff00);
        emu.cpu.pc = (h as u16) << 8 | (l as u16);
    } else {
        emu.cpu.pc = emu.mem.load_u16(r);
    }
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
    emu.cpu.sp = Flags::from_bits(stack::pop_u8(emu) | 0x20).unwrap();
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

pub fn cld_i(emu: &mut Emu) {
    emu.cpu.decimal_flag(false);
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
pub fn sed_i(emu: &mut Emu) {
    emu.cpu.decimal_flag(true);
    emu.cpu.pc += 1;
}
pub fn lda_imm(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn lda_zpg(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn lda_zpx(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn lda_abs(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn lda_abx(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn lda_aby(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn lda_inx(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn lda_iny(emu: &mut Emu) { 
    lda(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldx_imm(emu: &mut Emu) { 
    ldx(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldx_zpg(emu: &mut Emu) { 
    ldx(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldx_zpy(emu: &mut Emu) { 
    ldx(emu, emu.mem.load_u8(addressing::zeropage_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldx_abs(emu: &mut Emu) { 
    ldx(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn ldx_aby(emu: &mut Emu) { 
    ldx(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn ldy_imm(emu: &mut Emu) { 
    ldy(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldy_zpg(emu: &mut Emu) { 
    ldy(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldy_zpx(emu: &mut Emu) { 
    ldy(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn ldy_abs(emu: &mut Emu) { 
    ldy(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn ldy_abx(emu: &mut Emu) { 
    ldy(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn sta_zpg(emu: &mut Emu) { 
    sta(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn sta_zpx(emu: &mut Emu) { 
    sta(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn sta_abs(emu: &mut Emu) { 
    sta(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn sta_abx(emu: &mut Emu) { 
    sta(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn sta_aby(emu: &mut Emu) { 
    sta(emu, addressing::absolute_y(emu)); 
    emu.cpu.pc += 3;
}
pub fn sta_inx(emu: &mut Emu) { 
    sta(emu, addressing::indirect_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn sta_iny(emu: &mut Emu) { 
    sta(emu, addressing::indirect_y(emu)); 
    emu.cpu.pc += 2;
}
pub fn stx_zpg(emu: &mut Emu) { 
    stx(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn stx_zpy(emu: &mut Emu) { 
    stx(emu, addressing::zeropage_y(emu)); 
    emu.cpu.pc += 2;
}
pub fn stx_abs(emu: &mut Emu) { 
    stx(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn sty_zpg(emu: &mut Emu) { 
    sty(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn sty_zpx(emu: &mut Emu) { 
    sty(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn sty_abs(emu: &mut Emu) { 
    sty(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn adc_imm(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn adc_zpg(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn adc_zpx(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn adc_abs(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn adc_abx(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn adc_aby(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn adc_inx(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn adc_iny(emu: &mut Emu) { 
    adc(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn and_imm(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn and_zpg(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn and_zpx(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn and_abs(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn and_abx(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn and_aby(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn and_inx(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn and_iny(emu: &mut Emu) { 
    and(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn asl_zpg(emu: &mut Emu) { 
    asl(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn asl_zpx(emu: &mut Emu) { 
    asl(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn asl_abs(emu: &mut Emu) { 
    asl(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn asl_abx(emu: &mut Emu) { 
    asl(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn bit_zpg(emu: &mut Emu) { 
    bit(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn bit_abs(emu: &mut Emu) { 
    bit(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn cmp_imm(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn cmp_zpg(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn cmp_zpx(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn cmp_abs(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn cmp_abx(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn cmp_aby(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn cmp_inx(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn cmp_iny(emu: &mut Emu) { 
    cmp(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn cpx_imm(emu: &mut Emu) { 
    cpx(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn cpx_zpg(emu: &mut Emu) { 
    cpx(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn cpx_abs(emu: &mut Emu) { 
    cpx(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn cpy_imm(emu: &mut Emu) { 
    cpy(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn cpy_zpg(emu: &mut Emu) { 
    cpy(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn cpy_abs(emu: &mut Emu) { 
    cpy(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn dec_zpg(emu: &mut Emu) { 
    dec(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn dec_zpx(emu: &mut Emu) { 
    dec(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn dec_abs(emu: &mut Emu) { 
    dec(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn dec_abx(emu: &mut Emu) { 
    dec(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn eor_imm(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn eor_zpg(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn eor_zpx(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn eor_abs(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn eor_abx(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn eor_aby(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn eor_inx(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn eor_iny(emu: &mut Emu) { 
    eor(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn inc_zpg(emu: &mut Emu) { 
    inc(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn inc_zpx(emu: &mut Emu) { 
    inc(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn inc_abs(emu: &mut Emu) { 
    inc(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn inc_abx(emu: &mut Emu) { 
    inc(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn lsr_zpg(emu: &mut Emu) { 
    lsr(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn lsr_zpx(emu: &mut Emu) { 
    lsr(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn lsr_abs(emu: &mut Emu) { 
    lsr(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn lsr_abx(emu: &mut Emu) { 
    lsr(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn ora_imm(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn ora_zpg(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn ora_zpx(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn ora_abs(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn ora_abx(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn ora_aby(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn ora_inx(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn ora_iny(emu: &mut Emu) { 
    ora(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}
pub fn rol_zpg(emu: &mut Emu) { 
    rol(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn rol_zpx(emu: &mut Emu) { 
    rol(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn rol_abs(emu: &mut Emu) { 
    rol(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn rol_abx(emu: &mut Emu) { 
    rol(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn ror_zpg(emu: &mut Emu) { 
    ror(emu, addressing::zeropage(emu)); 
    emu.cpu.pc += 2;
}
pub fn ror_zpx(emu: &mut Emu) { 
    ror(emu, addressing::zeropage_x(emu)); 
    emu.cpu.pc += 2;
}
pub fn ror_abs(emu: &mut Emu) { 
    ror(emu, addressing::absolute(emu)); 
    emu.cpu.pc += 3;
}
pub fn ror_abx(emu: &mut Emu) { 
    ror(emu, addressing::absolute_x(emu)); 
    emu.cpu.pc += 3;
}
pub fn sbc_imm(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::immediate(emu))); 
    emu.cpu.pc += 2;
}
pub fn sbc_zpg(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::zeropage(emu))); 
    emu.cpu.pc += 2;
}
pub fn sbc_zpx(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::zeropage_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn sbc_abs(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::absolute(emu))); 
    emu.cpu.pc += 3;
}
pub fn sbc_abx(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::absolute_x(emu))); 
    emu.cpu.pc += 3;
}
pub fn sbc_aby(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::absolute_y(emu))); 
    emu.cpu.pc += 3;
}
pub fn sbc_inx(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::indirect_x(emu))); 
    emu.cpu.pc += 2;
}
pub fn sbc_iny(emu: &mut Emu) { 
    sbc(emu, emu.mem.load_u8(addressing::indirect_y(emu))); 
    emu.cpu.pc += 2;
}

