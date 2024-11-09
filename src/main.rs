#![allow(dead_code)]

mod rom;
mod registers;
mod render;
mod screen;
mod cpu;

extern crate sdl2;
extern crate spin_sleep;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


use render::Renderer;
use registers::Registers;
use rom::Rom;
use cpu::Cpu;
use screen::*;


const M_CYCLE_LENGTH: u32 = 1_000_000_000u32 / 1_048_576;
const FRAME_LENGTH: u32 = 17555;
const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;
const render_scale: u32 = 4;

fn main() {
	let arguments: Vec<String> = std::env::args().collect();
	assert!(arguments.len() == 2, "Not enough args");
	let rom: Rom = Rom::load_rom(arguments[1].clone()).ok()
		.expect("Failed to load Gameboy ROM");
	println!("File is a valid Gameboy ROM");

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("JAGE", SCREEN_WIDTH * render_scale, SCREEN_HEIGHT * render_scale)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cycle_count = 0;

    let mut renderer: Renderer = Renderer::new(window);
    let screen: Screen = Screen::new();

    let mut cpu: Cpu = Cpu::new(rom); 

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

        //CPU::

		if cycle_count == FRAME_LENGTH {
			cycle_count = 0;
			renderer.render(Screen::test_screen());
		}
        spin_sleep::sleep(Duration::new(0, M_CYCLE_LENGTH));
    }
}
