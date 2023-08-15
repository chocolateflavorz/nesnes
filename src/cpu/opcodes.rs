use crate::cpu::Cpu;
use crate::mem::Mem;
use lazy_static;

// instruction(registers, memory) -> cycles
pub type Behaviour = fn(&mut Cpu, &mut Mem) -> u8;

pub struct Op {
    pub behaviour: Behaviour,
    pub cycles: u8,
    pub mnemonic: &'static [u8; 3],
    pub opcode: u8,
}

pub fn brk(cpu: &mut Cpu, mem: &mut Mem) -> u8 {
    0
}

pub fn ora_indirectx(cpu: &mut Cpu, mem: &mut Mem) -> u8 {
    cpu.pc += 1;

    2
}



