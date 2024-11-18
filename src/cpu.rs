use crate::Registers;
use crate::Rom;
use crate::Screen;
use crate::registers::RegisterName;

pub struct Cpu {
    rom: Rom,
    registers: Registers,
    opcode: u8,
}

impl Cpu {
    pub fn new(rom: Rom) -> Cpu {
        let mut opcode = rom.data[0x0100];
        Cpu {
            rom,
            registers: Registers::new(),
            opcode
        }
    }
    pub fn exec(&mut self, screen: &mut Screen) -> i32 {//returns number of m-cycles to delay
        //println!("{:#04x}", self.opcode);
        let mut duration = 1; //most opcodes last 1 m-cycle
        let mut next_pc = self.registers.read(RegisterName::PC) + 1; //override with jump instructions
        match self.opcode {
            0x00 => {
            },
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                panic!("Invalid opcode!!! {:#04X?} at ${:#04X?}", self.opcode, next_pc - 1)
            }
            _ => {
                panic!("Unimplemented opcode!!! {:#04X?} at ${:#04X?}", self.opcode, next_pc - 1);
            }
        }
        self.registers.write(RegisterName::PC, next_pc);
        self.opcode = self.rom.data[next_pc as usize];//not right!!!!
        duration
    }
}