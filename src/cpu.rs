use crate::Registers;
use crate::Rom;
use crate::Screen;
use crate::registers::RegisterName;

#[derive(Copy, Clone)]
pub enum FlagStatus {
    NO_CHANGE,
    SET,
    RESET
}

pub struct Mbc {
    mbc_type: u8,
    active_bank: u16,
}

pub struct Cpu {
    rom: Rom,
    registers: Registers,
    mbc: Mbc,
    opcode: u8,
}

impl Cpu {
    pub fn new(rom: Rom) -> Cpu {
        let opcode = rom.data[0x0100];
        let mbc_type = rom.cartridge_type;
        Cpu {
            rom,
            registers: Registers::new(),
            mbc: Mbc {
                mbc_type,
                active_bank: 1,
            },
            opcode
        }
    }
    pub fn exec(&mut self, screen: &mut Screen) -> i32 {//returns number of m-cycles to delay
        let current_pc = self.registers.read(RegisterName::PC);
        let mut next_pc = current_pc; //override with jump instructions

        let mut duration = 1; //most opcodes last 1 m-cycle
        let mut length = 1;
        let mut flags = [FlagStatus::NO_CHANGE; 4];

        match self.opcode {
            0x00 => {//nop
            },
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => {//dec r8
                let register = match_register(self.opcode & 0b00111000 >> 3);
                let value = self.registers.read(register);
                self.registers.write(register, value - 1);

                let mut zero = FlagStatus::RESET;
                if value - 1 == 0 {
                    zero = FlagStatus::SET;
                }
                let mut half_carry = FlagStatus::RESET;
                if ((value & 0x0F) - 1) & 0x10 == 0x10 {
                    half_carry = FlagStatus::SET;
                }
                flags = [zero, FlagStatus::SET, half_carry, FlagStatus::NO_CHANGE]
            },
            0xB8..0xBD => {//cp a, r8
                let register = match_register(self.opcode & 0b00000111);
                let a = self.registers.read(RegisterName::A);
                let r8 = self.registers.read(register);

                let mut zero = FlagStatus::RESET;
                if a == r8 {
                    zero = FlagStatus::SET;
                }
                let mut half_carry = FlagStatus::RESET;
                if ((a & 0x0F) - (r8 & 0x0F)) & 0x10 == 0x10 {
                    half_carry = FlagStatus::SET;
                }
                let mut carry = FlagStatus::RESET;
                if a < r8 {
                    carry = FlagStatus::SET;
                }

                flags = [zero, FlagStatus::SET, half_carry, carry];
            },
            0xBF => {//cp a, a
                flags = [FlagStatus::SET, FlagStatus::SET, FlagStatus::RESET, FlagStatus::RESET];
            }
            0xC3 => {//jmp imm16
                next_pc = (self.read_from_memory(current_pc + 1) as u16) + (self.read_from_memory(current_pc + 2) as u16) << 8;
                length = 3;
                duration = 3;
            },
            0xC6 => {//add a, imm8
                let a = self.registers.read(RegisterName::A);
                let imm8 = self.read_from_memory(current_pc + 1) as u16;
                self.registers.write(RegisterName::A, a + imm8 as u16);

                let mut zero = FlagStatus::RESET;
                if a + imm8 == 0 {
                    zero = FlagStatus::SET;
                }
                let mut half_carry = FlagStatus::RESET;
                if (a & 0x0F) + (imm8 & 0x0F) & 0x10 == 0x10 {
                    half_carry = FlagStatus::SET;
                }
                let mut carry = FlagStatus::RESET;
                if a + imm8 > 0xFF {
                    carry = FlagStatus::SET;
                }

                length = 2;
                duration = 2;
                flags = [zero, FlagStatus::RESET, half_carry, carry];
            }
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                panic!("Invalid opcode!!! {:#04X?} at ${:#06X?}", self.opcode, next_pc - 1);
            }
            _ => {
                panic!("Unimplemented opcode!!! {:#04X?} at {:#06X?}", self.opcode, next_pc - 1);
            }
        }
        if next_pc == current_pc {
            next_pc += length;
        }
        self.registers.write(RegisterName::PC, next_pc);
        self.opcode = self.read_from_memory(next_pc);
        duration
    }
    fn read_from_memory(&mut self, address: u16) -> u8 {
        match address {
            0x0000..0x3FFF => {
                match self.rom.cartridge_type {
                    0x13 => {
                        return self.rom.data[address as usize];
                    }
                    _=> {
                        panic!("Unimplemented MBC!!! {:#04X?}", self.rom.cartridge_type)
                    }
                }
            }
            0x4000..0x7FFF => {
                match self.rom.cartridge_type {
                    0x13 => {
                        return self.rom.data[(address + 0x4000 * (self.mbc.active_bank - 1)) as usize];
                    }
                    _=> {
                        panic!("Unimplemented MBC!!! {:#04X?}", self.rom.cartridge_type)
                    }
                }
            }
            _ => {
                panic!("Unimplemented read!!! {:#06X?}", address);
            }
        }
    }

}

fn match_register(operand: u8) -> RegisterName {
    match operand {
        0 => RegisterName::B,
        1 => RegisterName::C,
        2 => RegisterName::D,
        3 => RegisterName::E,
        4 => RegisterName::H,
        5 => RegisterName::L,
        7 => RegisterName::A,
        _ => panic!("Invalid register!!! {}", operand)
    }
}