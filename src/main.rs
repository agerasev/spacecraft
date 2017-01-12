extern crate sdl2;
extern crate gl;

use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};

fn main() {
	let width = 800;
	let height = 600;

	let sdl_context = sdl2::init().unwrap();
	let video_subsys = sdl_context.video().unwrap();
	let window = video_subsys.window("SpaceCraft", width, height).position_centered().opengl().build().unwrap();

	let context = window.gl_create_context().unwrap();
	gl::load_with(|name| video_subsys.gl_get_proc_address(name) as *const _);

	let mut events = sdl_context.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				Event::KeyDown{keycode, ..} => if keycode.unwrap() == Keycode::Escape { break 'main; },
				_ => continue,
			}
		}

		unsafe {
			gl::ClearColor(0.0, 1.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		window.gl_swap_window();
	}
}
