use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Flags: u8 {
        const carry = 0b00000001;
        const zero  = 0b00000010;
        const int   = 0b00000100;
        const dec   = 0b00001000;
        const b     = 0b00110000;
        const over  = 0b01000000;
        const neg   = 0b10000000;
    }
}

pub struct Cpu {
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    p: Flags,
}