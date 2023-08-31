#![cfg_attr(rustfmt, rustfmt_skip)]

use std::ops::Shl;

use crate::cpu::Cpu;
use crate::mem::Mem;

// instruction(registers, memory) -> additional cycles
pub type Behaviour = fn(&mut Cpu, &mut Mem) -> u8;

pub const OP_FUNC: [Behaviour; 0] = [];
pub const OP_CYCLE: [u8; 0] = [];
pub const OP_NAME: [&'static str; 0] = [];
pub const OP_LEN: [u8; 0] = [];

#[inline] 
fn immediate(cpu: &Cpu, _mem: &Mem) -> u16 { 
    cpu.pc 
}
#[inline] 
fn zeropage(cpu: &Cpu, mem: &Mem) -> u16 { 
    mem.load_u8(cpu.pc) as u16 
}
#[inline]
fn zeropage_x(cpu: &Cpu, mem: &Mem) -> u16 {
    mem.load_u8(cpu.pc).wrapping_add(cpu.x) as u16 
}
#[inline]
fn zeropage_y(cpu: &Cpu, mem: &Mem) -> u16 {
    mem.load_u8(cpu.pc).wrapping_add(cpu.y) as u16 
}
#[inline] 
fn absolute(cpu: &Cpu, mem: &Mem) -> u16 {
    mem.load_u16(cpu.pc) 
}
#[inline] 
fn absolute_x(cpu: &Cpu, mem: &Mem) -> u16 {
    mem.load_u16(mem.load_u16(cpu.pc)).wrapping_add(cpu.x as u16) 
}
#[inline]
fn absolute_y(cpu: &Cpu, mem: &Mem) -> u16 {
    mem.load_u16(mem.load_u16(cpu.pc)).wrapping_add(cpu.y as u16) 
}
#[inline]
fn indirect_x(cpu: &Cpu, mem: &Mem) -> u16 {
    let addr = mem.load_u8(cpu.pc).wrapping_add(cpu.x);
    mem.load_u16(addr as u16)
}
#[inline]
fn indirect_y(cpu: &Cpu, mem: &Mem) -> u16 {
    let addr = mem.load_u8(cpu.pc);
    mem.load_u16(addr as u16).wrapping_add(cpu.y as u16)
}


#[inline]
fn brk(cpu: &mut Cpu, mem: &mut Mem) { }
#[inline]
fn ora(cpu: &mut Cpu, val: u8) { 
    cpu.a = cpu.a | val;   
    cpu.nz_flags(cpu.a) 
}
#[inline]
fn adc(cpu: &mut Cpu, val: u8) {
    let (r, v) = cpu.a.overflowing_add(val);
    cpu.a = r;
    cpu.carry(v);
    cpu.overflow(v);
    cpu.nz_flags(cpu.a);
}
#[inline] 
fn and(cpu: &mut Cpu, val: u8) { 
    cpu.a = cpu.a & val;   
    cpu.nz_flags(cpu.a) 
}
#[inline]
fn asl(cpu: &mut Cpu, val: u8) {
    cpu.carry(val & 0x40 != 0);
    cpu.a = (val as i8).shl(1) as u8;
    cpu.nz_flags(cpu.a);
}
#[inline]
fn lda (cpu: &mut Cpu, val: u8) {
    cpu.a = val;
    cpu.nz_flags(cpu.a);
}
#[inline]
fn ldx (cpu: &mut Cpu, val: u8) {
    cpu.x = val;
    cpu.nz_flags(cpu.x);
}
#[inline]
fn ldy (cpu: &mut Cpu, val: u8) {
    cpu.y = val;
    cpu.nz_flags(cpu.y);
}
#[inline]
fn sta (cpu: &Cpu, mem: &mut Mem, val: u8) {
    mem.store_u8(val, cpu.a);
}
#[inline]
fn stx (cpu: &Cpu, mem: &mut Mem, val: u8) {
    mem.store_u8(val, cpu.x);
}
#[inline]
fn sty (cpu: &Cpu, mem: &mut Mem, val: u8) {
    mem.store_u8(val, cpu.y);
}
fn tax_i (cpu: &mut Cpu) {
    cpu.x = cpu.a;
    cpu.nz_flags(cpu.a);
}
fn tay_i (cpu: &mut Cpu) {
    cpu.y = cpu.a;
    cpu.nz_flags(cpu.a);
}
fn tsx_i (cpu: &mut Cpu) {
    cpu.x = cpu.sp;
    cpu.nz_flags(cpu.sp);
}
fn txa_i (cpu: &mut Cpu) {
    cpu.a = cpu.x;
    cpu.nz_flags(cpu.x);
}




fn ora_ix(cpu: &mut Cpu, mem: &mut Mem) { 
    ora(cpu, mem.load_u8(indirect_x(cpu, mem))); 
    cpu.pc += 2
}
fn ora_zp(cpu: &mut Cpu, mem: &mut Mem) { 
    ora(cpu, mem.load_u8(zeropage(cpu, mem))); 
    cpu.pc += 2
}
