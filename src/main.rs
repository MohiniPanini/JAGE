mod rom;
mod cpu;

fn main() {
	let arguments: Vec<String> = std::env::args().collect();
	assert!(arguments.len() == 2, "Not enough args");
	let game_rom: rom::Rom = rom::load_rom(arguments[1].clone()).ok()
		.expect("Failed to load Gameboy ROM");
	println!("File is a valid Gameboy ROM");
	
	let mut registers = cpu::Registers::init();
	registers.write(cpu::RegisterName::A, 128);
	registers.write(cpu::RegisterName::F, 1);
	println!("{:}", registers.read(cpu::RegisterName::AF)); //0b1000000000000001
}
