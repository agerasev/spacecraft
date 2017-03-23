extern crate num;
extern crate sdl2;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate linalg as la;

#[allow(dead_code)]
#[macro_use]
mod core;

#[allow(dead_code)]
#[macro_use]
mod view;

use std::f64::consts::PI;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode};
use sdl2::mouse::{MouseButton};

use glium::{Surface};

use glium_sdl2::DisplayBuild;

use la::vec::*;
use la::mat::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

use core::pos::*;
use core::ori::*;
use core::map::*;
use core::block;

use view::engine::Engine;
/*
use view::planet::Planet;
use view::draw::Draw;
*/

fn main() {
	let width = 600;
	let height = 600;

	let sdl_context = sdl2::init().unwrap();
	let video_subsys = sdl_context.video().unwrap();
	let display = video_subsys.window("SpaceCraft", width, height).position_centered().opengl().resizable().build_glium().unwrap();

	/*
	unsafe {
		gl::Enable(gl::DEPTH_TEST);
		gl::DepthFunc(gl::LESS);
		
		gl::Enable(gl::CULL_FACE);
		gl::CullFace(gl::BACK);
	}
	*/

	let mut engine = Engine::new();
	engine.load_program("array", "array.vs", "array.fs").unwrap();
	/*
	let mut planet = Planet::new(3, 3);

	let mut phi: f64 = 0.0;
	let mut theta: f64 = PI/4.0;
	let mut rad: f64 = (planet.size().sqr() as f64).sqrt()*block::SIZE;

	let sens: f64 = 1e-2;
	let zoom: f64 = 1.2;
	let eps: f64 = 2e-3;

	let mut lmb = false;
	*/

	let vertex1 = Vertex { position: [-0.5, -0.5] };
	let vertex2 = Vertex { position: [ 0.0,  0.5] };
	let vertex3 = Vertex { position: [ 0.5, -0.25] };
	let shape = vec![vertex1, vertex2, vertex3];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vertex_shader_src = r#"
		#version 140

		in vec2 position;

		void main() {
			gl_Position = vec4(position, 0.0, 1.0);
		}
	"#;

	let fragment_shader_src = r#"
		#version 140

		out vec4 color;

		void main() {
			color = vec4(1.0, 0.0, 0.0, 1.0);
		}
	"#;

	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	let mut events = sdl_context.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				/*
				Event::KeyDown{keycode, ..} => if keycode.unwrap() == Keycode::Escape { break 'main; },
				Event::MouseButtonDown{mouse_btn, ..} => if mouse_btn == MouseButton::Left { lmb = true; },
				Event::MouseButtonUp{mouse_btn, ..} => if mouse_btn == MouseButton::Left { lmb = false; },
				Event::MouseMotion{xrel, yrel, ..} => {
					if lmb {
						phi += (xrel as f64)*sens;
						theta += -(yrel as f64)*sens;
						if theta < eps { theta = eps; }
						if theta > PI - eps { theta = PI - eps; }
					}
				},
				Event::MouseWheel{y, ..} => {
					rad *= zoom.powi(-(y as i32));
				},
				Event::Window{win_event, ..} => {
					match win_event {
						WindowEvent::Resized(w, h) => {
							unsafe { gl::Viewport(0, 0, w, h); }
							engine.camera.aspect(w as f64 / h as f64);
						},
						_ => {},
					} 
				},
				*/
				_ => {},
			}
		}

		/*
		let dir = vec3d::from([theta.sin()*phi.cos(), theta.sin()*phi.sin(), theta.cos()]);
		let right = dir.cross(vec3d::from([0.0, 0.0, 1.0])).normalize();
		let top = right.cross(dir).normalize();
		let front = dir;
		engine.camera.set_pos(dir*rad);
		engine.camera.set_ori(mat3d::from([
			right[0], top[0], front[0], 
			right[1], top[1], front[1], 
			right[2], top[2], front[2], 
		]));

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
		
		planet.draw(&mut engine).unwrap();

		unsafe { gl::Flush(); }

		window.gl_swap_window();
		*/

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
		target.finish().unwrap();
	}
}
