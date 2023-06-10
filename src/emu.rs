use crate::cpu::Cpu;

const MEMORY_SIZE: usize = 64 * 1024;
struct Emulator {
    cpu: Cpu,
    mem: [u8; MEMORY_SIZE],
}

pub fn run(bin: &Vec<u8>) {

}