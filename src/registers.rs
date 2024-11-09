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
	SP,
	PC,
	SPPC //not a real register!!
}

pub struct Registers {
	af: RegisterUnion,
	bc: RegisterUnion,
	de: RegisterUnion,
	sppc: RegisterUnion
}


impl Registers {
	pub fn new() -> Registers {
		return Registers {
			af: RegisterUnion {double: 0},
			bc: RegisterUnion {double: 0},
			de: RegisterUnion {double: 0},
			sppc: RegisterUnion {double: 0}
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

				RegisterName::SP => self.sppc.singles[1] = data as u8,
				RegisterName::PC => self.sppc.singles[0] = data as u8,
				RegisterName::SPPC => panic!("No valid opcodes support SPPC (I think)")
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
	
				RegisterName::SP => self.sppc.singles[1] as u16,
				RegisterName::PC => self.sppc.singles[0] as u16,
				RegisterName::SPPC => panic!("No valid opcodes support SPPC (I think)")
			}
		}
	}
}


