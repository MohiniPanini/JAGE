use std::fmt;

union RegisterUnion {
	double: u16,
	singles: [u8; 2] //index 0 is least significant byte of double, index 1 is most significant
}
#[derive(Copy, Clone)]
pub enum RegisterName {
	A,
	F,
	AF,
	B,
	C,
	BC,
	D,
	E,
	DE,
	H,
	L,
	HL,
	SP,
	PC
}

pub struct Registers {
	af: RegisterUnion,
	bc: RegisterUnion,
	de: RegisterUnion,
	hl: RegisterUnion,
	sp: u16,
	pc: u16
}


impl Registers {
	pub fn new() -> Registers {
		return Registers {//emulates behavior of DMG
			af: RegisterUnion {double: 0x01B0},
			bc: RegisterUnion {double: 0x0013},
			de: RegisterUnion {double: 0x00D8},
			hl: RegisterUnion {double: 0x014D},
			sp: 0xFFFE,
			pc: 0x0100
		}
	}	
	pub fn write(&mut self, register: RegisterName, data: u16) {
		unsafe {
			match register {
				RegisterName::A => self.af.singles[1] = data as u8,
				RegisterName::F => self.af.singles[0] = data as u8,
				RegisterName::AF => self.af.double = data,

				RegisterName::B => self.bc.singles[1] = data as u8,
				RegisterName::C => self.bc.singles[0] = data as u8,
				RegisterName::BC => self.bc.double = data,

				RegisterName::D => self.de.singles[1] = data as u8,
				RegisterName::E => self.de.singles[0] = data as u8,
				RegisterName::DE => self.de.double = data,

				RegisterName::H => self.hl.singles[1] = data as u8,
				RegisterName::L => self.hl.singles[0] = data as u8,
				RegisterName::HL => self.hl.double = data,

				RegisterName::SP => self.sp = data,
				RegisterName::PC => self.pc = data,
			}
		}
	}
	pub fn read(&self, register: RegisterName) -> u16 {
		unsafe {
			match register {
				RegisterName::A => self.af.singles[1] as u16,
				RegisterName::F => self.af.singles[0] as u16,
				RegisterName::AF => self.af.double,

				RegisterName::B => self.bc.singles[1] as u16,
				RegisterName::C => self.bc.singles[0] as u16,
				RegisterName::BC => self.bc.double,
	
				RegisterName::D => self.de.singles[1] as u16,
				RegisterName::E => self.de.singles[0] as u16,
				RegisterName::DE => self.de.double,

				RegisterName::H => self.hl.singles[1] as u16,
				RegisterName::L => self.hl.singles[0] as u16,
				RegisterName::HL => self.hl.double,
	
				RegisterName::SP => self.sp,
				RegisterName::PC => self.pc,
			}
		}
	}
}

impl fmt::Display for Registers {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let a = self.read(RegisterName::A);
		let f = self.read(RegisterName::F);        
        let b = self.read(RegisterName::B);
        let c = self.read(RegisterName::C);        
        let d = self.read(RegisterName::D);
        let e = self.read(RegisterName::E);        
        let h = self.read(RegisterName::H);
        let l = self.read(RegisterName::L);        
        let sp = self.read(RegisterName::SP);
        let pc = self.read(RegisterName::PC);
        
        writeln!(formatter, "AF: {:08b}\t{:08b}\t{:#04X?}\t{:#04X?}", a, f, a, f)?;
        writeln!(formatter, "BC: {:08b}\t{:08b}\t{:#04X?}\t{:#04X?}", b, c, b, c)?;
        writeln!(formatter, "DE: {:08b}\t{:08b}\t{:#04X?}\t{:#04X?}", d, e, d, e)?;
        writeln!(formatter, "HL: {:08b}\t{:08b}\t{:#04X?}\t{:#04X?}", h, l, h, l)?;
        writeln!(formatter, "SP: {:#04X?}", sp)?;
        writeln!(formatter, "PC: ${:04X?}", pc)
        
    }
}