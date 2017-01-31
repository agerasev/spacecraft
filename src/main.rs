extern crate sdl2;
extern crate gl4u;
extern crate linalg as la;

#[allow(dead_code)]
#[macro_use]
mod core;

#[allow(dead_code)]
#[macro_use]
mod view;

use std::f64::consts::PI;

use la::vec::*;
use la::mat::*;

use gl4u::gl;

use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::mouse::{MouseButton};

use core::pos::*;
use core::ori::*;

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

	unsafe {
		gl::Enable(gl::DEPTH_TEST);
		gl::DepthFunc(gl::LESS);
		
		gl::Enable(gl::CULL_FACE);
		gl::CullFace(gl::FRONT);
	}

	let mut engine = Engine::new().load_program("main", "main.vs", "main.fs").unwrap();
	let mut planet = Planet::new(32);

	let mut phi: f64 = 0.0;
	let mut theta: f64 = PI/4.0;
	let mut rad: f64 = 3.0;

	let sens: f64 = 1e-2;
	let zoom: f64 = 1.2;
	let eps: f64 = 2e-3;

	let mut lmb = false;

	let mut events = sdl_context.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
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
				_ => continue,
			}
		}

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
		
		planet.draw(&engine.handle()).unwrap();

		unsafe { gl::Flush(); }

		window.gl_swap_window();
	}
}
