extern crate sdl2;
extern crate gl4u;
extern crate linalg as la;

use la::mat::*;

mod core;
mod view;

use gl4u::gl;
use gl4u::gl::types::*;

use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};

use view::engine::Engine;
use view::planet::Planet;
use view::draw::Draw;

fn main() {
	let width = 600;
	let height = 600;

	let sdl_context = sdl2::init().unwrap();
	let video_subsys = sdl_context.video().unwrap();
	let window = video_subsys.window("SpaceCraft", width, height).position_centered().opengl().build().unwrap();

	let _context = window.gl_create_context().unwrap();
	gl::load_with(|name| video_subsys.gl_get_proc_address(name) as *const _);

	let engine = Engine::new().load_program("main", "main.vs", "main.fs").unwrap();
	let planet = Planet::new();

	let mut phi: f32 = 0.0;
	let mut events = sdl_context.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				Event::KeyDown{keycode, ..} => if keycode.unwrap() == Keycode::Escape { break 'main; },
				_ => continue,
			}
		}

		let view = mat4::<GLfloat>::from([
			0.5*phi.cos(), 0.5*phi.sin(), 0.0, 0.0,
			-0.5*phi.sin(), 0.5*phi.cos(), 0.0, 0.0,
			0.0, 0.0, 0.1, 0.0,
			0.0, 0.0, 0.0, 1.0
		]);

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
			planet.draw(&engine)
				.uniform_matrix("view", view.data()).unwrap()
				.draw().unwrap();

		unsafe { gl::Flush(); }

		phi += 0.01;

		window.gl_swap_window();
	}
}
