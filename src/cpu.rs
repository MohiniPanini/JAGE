use crate::Registers;
use crate::Rom;
use crate::Screen;
use crate::registers::RegisterName;

const ZERO_FLAG: u8 = 0b10000000;
const SUB_FLAG: u8 = 0b01000000;
const HALF_CARRY_FLAG: u8 = 0b00100000;
const CARRY_FLAG: u8 = 0b00010000;

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
        let mut f = self.registers.read(RegisterName::F) as u8;

        let mut duration = 1; //most opcodes last 1 m-cycle
        let mut length = 1;
        
        println!("Executing opcode {:#04X?} at ${:04X?}", self.opcode, current_pc);
        println!("{}", self.registers);

        match self.opcode {
            0x00 => {//nop
            },
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C => {//inc r8
                let register = match_register_u8(self.opcode & 0b00111000 >> 3);
                let r8 = self.registers.read(register);
                self.registers.write(register, r8.wrapping_add(1));

                let mask = f & 0b00010000;
                f = set_add_flags_u8(r8 as u8, 1, f);
                if mask != 0 {
                    f = f | mask;
                }
                else {
                    f = f & 0b11101111;
                }
            }
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => {//dec r8
                let register = match_register_u8(self.opcode & 0b00111000 >> 3);
                let r8 = self.registers.read(register);
                self.registers.write(register, r8.wrapping_sub(1));

                let mask = f & 0b00010000;
                f = set_sub_flags_u8(r8 as u8, 1, f);
                if mask != 0 {
                    f = f | mask;
                }
                else {
                    f = f & 0b11101111;
                }
            },
            0x20 | 0x28 | 0x30 | 0x38 => {//jr cc,e8
                let cond = (self.opcode & 0b00011000) >> 3;
                let jump: bool = match cond {
                    0 => {
                        (f & 0b10000000) == 0
                    },
                    1 => {
                        (f & 0b10000000) != 0
                    },
                    2 => {
                        (f & 0b00010000) == 0
                    },
                    3 => {
                        (f & 0b00010000) != 0
                    },
                    _ => {
                        false
                    }
                };
                duration = 2;
                if jump {
                    duration = 3;
                    next_pc = current_pc + self.read_from_memory(current_pc.wrapping_add(1)) as u16;
                }
                length = 2;
            }
            0x40..=0x7F => {//ld r8, r8
                if self.opcode == 0x76 {//halt
                    loop {
                        
                    }
                }
                
                let op1 = self.opcode & 0b00000111;
                let mut src: u8 = 0;
                if op1 == 0b0110 {
                    src = self.read_from_memory(self.registers.read(RegisterName::HL));
                    duration = 2;
                }
                else {
                    src = self.registers.read(match_register_u8(op1)) as u8;
                }
                
                let op2 = (self.opcode & 0b00111000) >> 3;
                if op2 == 0b0110 {
                    self.write_to_memory(self.registers.read(RegisterName::HL), src);
                    duration = 2;
                }
                else {
                    self.registers.write(match_register_u8(op2), src as u16);
                }
            }
            0xB8..=0xBE => {//cp a, r8
                let register = match_register_u8(self.opcode & 0b00000111);
                let a = self.registers.read(RegisterName::A) as u8;
                let r8 = self.registers.read(register) as u8;

                f = set_sub_flags_u8(a, r8, f);
            },
            0xBF => {//cp a, a
                f = 0b11000000;
            }
            0xC3 => {//jmp imm16
                next_pc = (self.read_from_memory(current_pc.wrapping_add(1)) as u16) + (self.read_from_memory(current_pc.wrapping_add(2)) as u16) << 8;
                length = 3;
                duration = 3;
            },
            0xC6 => {//add a, imm8
                let a = self.registers.read(RegisterName::A) as u8;
                let imm8 = self.read_from_memory(current_pc.wrapping_add(1));
                self.registers.write(RegisterName::A, (a.wrapping_add(imm8)) as u16);

                f = set_add_flags_u8(a, imm8, f);

                length = 2;
                duration = 2;
            }
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                panic!("Invalid opcode!!! {:#04X?} at ${:04X?}", self.opcode, current_pc);
            }
            0xFA => {
                                    
            }
            _ => {
                panic!("Unimplemented opcode!!! {:#04X?} at ${:04X?}", self.opcode, current_pc);
            }
        }

        self.registers.write(RegisterName::F, f as u16);
        
        if next_pc == current_pc {
            next_pc = next_pc.wrapping_add(length);
        }
        self.registers.write(RegisterName::PC, next_pc);
        self.opcode = self.read_from_memory(next_pc);
        duration
    }
    
    fn read_from_memory(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
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
                        return self.rom.data[(address.wrapping_add(0x4000 * (self.mbc.active_bank - 1))) as usize];
                    }
                    _=> {
                        panic!("Unimplemented MBC!!! {:#04X?}", self.rom.cartridge_type)
                    }
                }
            }
            _ => {
                panic!("Unimplemented read!!! ${:04X?}", address);
            }
        }
    }
    
    fn write_to_memory(&mut self, address: u16, data: u8) {
        panic!("Unimplemented write!!! ${:04X?}", address);
    }    

}

fn match_register_u8(op: u8) -> RegisterName {
    match op {
        0 => RegisterName::B,
        1 => RegisterName::C,
        2 => RegisterName::D,
        3 => RegisterName::E,
        4 => RegisterName::H,
        5 => RegisterName::L,
        7 => RegisterName::A,
        _ => panic!("Invalid register!!! {}", op)
    }
}

fn set_add_flags_u8(op1: u8, op2: u8, f: u8) -> u8 {
    let mut flags = f;
    
    if op1.wrapping_add(op2) == 0 {
        flags = flags | ZERO_FLAG;
    }
    
    flags = flags & !SUB_FLAG;
    
    if (op1 & 0x0F).wrapping_add(op2 & 0x0F) & 0x10 == 0x10 {
        flags = flags | HALF_CARRY_FLAG;
    }
    
    if op1 as u16 + op2 as u16 > 0xFF {
        flags = flags | CARRY_FLAG;
    }
    
    flags
}

fn set_sub_flags_u8(op1: u8, op2: u8, f: u8) -> u8 {
    let mut flags = f;
    
    if op1.wrapping_sub(op2) == 0 {
        flags = flags | ZERO_FLAG;
    }
    
    flags = flags | SUB_FLAG;
    
    if (op1 & 0x0F).wrapping_sub(op2 & 0x0F) & 0x10 == 0x10 {
        flags = flags | HALF_CARRY_FLAG;
    }
    
    if op1 < op2 {
        flags = flags | CARRY_FLAG;
    }
    
    flags
}