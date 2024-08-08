mod rom;
mod cpu;
mod render;
mod tilemap;

extern crate sdl2;

use render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::time::Instant; //DEBUG
use cpu::RegisterName;
use cpu::Registers;

const M_CYCLE_LENGTH: u32 = 1_000_000_000u32 / 1_048_576;
const FRAME_LENGTH: u32 = 240; //idk why this is right but don't change it
const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;
const render_scale: u32 = 4;

fn main() {
	let arguments: Vec<String> = std::env::args().collect();
	assert!(arguments.len() == 2, "Not enough args");
	let game_rom: rom::Rom = rom::load_rom(arguments[1].clone()).ok()
		.expect("Failed to load Gameboy ROM");
	println!("File is a valid Gameboy ROM");
	
	let mut registers = Registers::new();
	registers.write(RegisterName::A, 128);
	registers.write(RegisterName::F, 1);
	println!("{:}", registers.read(RegisterName::AF)); //0b1000000000000001

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("JAGE", SCREEN_WIDTH * render_scale, SCREEN_HEIGHT * render_scale)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer: Renderer = Renderer::new(window);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cycle_count = 0;

    /*
    let now = Instant::now(); //DEBUG
    let mut elapsed = now.elapsed(); //DEBUG
    let mut framecount = 0; //DEBUG
    */

    'running: loop {
        cycle_count = cycle_count + 1;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

		if cycle_count == FRAME_LENGTH {
            /*
            elapsed = now.elapsed();
            framecount = framecount + 1;
            */
			cycle_count = 0;
			renderer.render();
		}
        ::std::thread::sleep(Duration::new(0, M_CYCLE_LENGTH));
    }
    //println!("Rendered frame {:}, time elapsed: {:.2?}", framecount, elapsed);
}
