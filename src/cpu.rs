use crate::Registers;
use crate::Rom;

pub struct Cpu {
    rom: Rom,
    registers: Registers,

}

impl Cpu {
    pub fn new(rom: Rom) -> Cpu {
        Cpu {
            rom,
            registers: Registers::new(),
        }
    }
}