extern crate sdl2;
extern crate gl4u;

use gl4u::gl;
use gl4u::shader::{Shader, Type};
use gl4u::program::{Program};
use gl4u::buffer::{Buffer};

use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};

#[allow(unused_variables)]
fn main() {
	let width = 600;
	let height = 600;

	let sdl_context = sdl2::init().unwrap();
	let video_subsys = sdl_context.video().unwrap();
	let window = video_subsys.window("SpaceCraft", width, height).position_centered().opengl().build().unwrap();

	let context = window.gl_create_context().unwrap();
	gl::load_with(|name| video_subsys.gl_get_proc_address(name) as *const _);

	let (vs, log) = Shader::new(Type::Vertex).load_file("res/shaders/main.vs").unwrap().compile().unwrap();
	if log.len() > 0 { println!("{}", log); }
	let (fs, log) = Shader::new(Type::Fragment).load_file("res/shaders/main.fs").unwrap().compile().unwrap();
	if log.len() > 0 { println!("{}", log); }

	let prog = Program::new().attach(vs).attach(fs).link().unwrap();

	let mut coord = Buffer::new(3);
	coord.load_float(&[0.0, 1.0, 0.0,  -(3.0 as f32).sqrt()/2.0, -0.5, 0.0,  (3.0 as f32).sqrt()/2.0, -0.5, 0.0]);
	let mut color = Buffer::new(3);
	color.load_float(&[1.0, 0.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0, 1.0]);

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

		let model: &[f32] = &[
			phi.cos(), phi.sin(), 0.0,
			-phi.sin(), phi.cos(), 0.0,
			0.0, 0.0, 1.0,
		];

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			prog.use_().unwrap()
				.attribute("position", &coord).unwrap()
				.attribute("color", &color).unwrap()
				.uniform_matrix("model", model).unwrap()
				.draw(0, 3).unwrap();

			gl::Flush();
		}

		phi += 0.01;

		window.gl_swap_window();
	}
}
