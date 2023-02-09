use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind, Seek, SeekFrom};

const ROM_BANK_SIZE: u64 = 32768;
const VALID_LOGO: [u8; 48] = [0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 
							0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 
							0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];

#[derive(Default)]
#[derive(Debug)]
pub struct Rom {
	data: Vec<u8>,
	
	//all following fields should be slices of data
	title: [u8; 11],
	manufacturing_code: [u8; 4],
	cgb: u8,
	licensee_code: [u8; 2],
	sgb: u8,
	cartridge_type: u8,
	rom_size: u8,
	ram_size: u8
}

pub fn load_rom(filename: String) -> Result<Rom, Error> {
	let mut file = File::open(filename)?;
	let filesize = file.seek(SeekFrom::End(0)).expect("Invalid filesize");
	file.seek(SeekFrom::Start(0));
	if filesize == 0 || filesize % ROM_BANK_SIZE != 0 {
		Error::new(ErrorKind::Other, "File is not a valid Gameboy ROM, or is corrupt");
	}
	let mut data: Vec<u8> = Vec::new();
	file.read_to_end(&mut data)?;
	println!("{:}", data.len());
	
	//verification process
	let logo = &data[0x0104..0x0134];
	for i in 0..48 {
		if logo[i] != VALID_LOGO[i] {
			Error::new(ErrorKind::Other, "Logo match failed");
		}
	}
	let mut checksum: i32 = 0;
	let checksum_bytes = &data[0x0134..0x014D];
	for i in checksum_bytes.iter().cloned() {
		let byte: i32 = i.try_into().unwrap();
		checksum -= byte + 1;
	}
	checksum &= 0xFF;
	if checksum != data[0x014D].try_into().unwrap() {
		Error::new(ErrorKind::Other, "Checksum match failed");
	}
	
	//initialize struct
	let mut title: [u8; 11] = [0; 11];
	title[..].clone_from_slice(&data[0x0134..(0x0134)+11]);
	let mut manufacturing_code: [u8; 4] = [0; 4];
	manufacturing_code[..].clone_from_slice(&data[0x013F..(0x013F)+4]);
	
	let mut licensee_code: [u8; 2] = [0, 0];
	if data[0x014B] == 0x0033 {
        licensee_code[0] = data[0x0144];
        licensee_code[1] = data[0x0145];
    }
    else {
        licensee_code[1] = data[0x014B];
	}
	Ok(Rom {
		title: title,
		manufacturing_code: manufacturing_code,
		cgb: data[0x0134],
		licensee_code: licensee_code,
        sgb: data[0x0146],
        cartridge_type: data[0x0147],
        rom_size: data[0x0148],
        ram_size: data[0x0149],
        data: data
	})
}
