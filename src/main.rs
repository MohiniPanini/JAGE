mod rom;

fn main() {
	
	let arguments: Vec<String> = std::env::args().collect();
	if arguments.len() < 2 {
		panic!("Not enough args");
	}
	let game_rom: rom::Rom = rom::load_rom(arguments[1].clone()).ok()
		.expect("Failed to load Gameboy ROM");
	println!("File is a valid Gameboy ROM");
}
