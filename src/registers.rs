union RegisterUnion {
	double: u16,
	singles: [u8; 2] //index 0 is least significant byte of double, index 1 is most significant
}

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
			af: RegisterUnion {double: 0x0108},
			bc: RegisterUnion {double: 0x0013},
			de: RegisterUnion {double: 0x00D8},
			hl: RegisterUnion {double: 0x007C},
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


