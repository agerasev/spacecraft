use std::fs::File;
use std::collections::{HashMap};

use glium::{Program, VertexBuffer};
use glium::backend::Facade;

use view::model::Model;
use view::camera::Camera;
use view::error::Error;

pub struct Engine<'a> {
	display: &'a Facade,
	shaders: HashMap<String, String>,
	programs: HashMap<String, Program>,
	//buffers: HashMap<String, VertexBuffer>,
	pub camera: Camera,
}

impl<'a> Engine<'a> {
	pub fn new(display: &'a Facade) -> Self {
		Engine { display: display, shaders: HashMap::new(), programs: HashMap::new(), buffers: HashMap::new(), camera: Camera::new() }
	}

	pub fn load_shader(&mut self, filename: &str) -> Result<&str, Error> {
		match self.shaders.get(filename) {
			Ok(src) => { return src; },
			Err(..) => {},
		}

		let src = try!(match File::open(filename) {
			Ok(f) => {
				self.raw.name = String::from(filename);
				let mut file = f;
				let mut content = String::new();
				try!(file.read_to_string(&mut content));
				Ok(content)
			},
			Err(e) => Err(Error::new(String::new() + filename + ": " + e.description()))
		});

		self.shaders.insert(filename.to_string(), src);
		self.shaders.get(filename).unwrap()
	}

	pub fn load_program(&mut self, name: &str, vsfn: &str, fsfn: &str) -> Result<(), Error> {
		let vs = try!(self.load_shader(&("res/shaders/".to_string() + vsfn)));
		let fs = try!(self.load_shader(&("res/shaders/".to_string() + fsfn)));
		let prog = try!(Program::from_source(self.display, vs, fs, None));
		self.programs.insert(name.to_string(), prog);
		Ok(())
	}

	pub fn get_program(&self, name: &str) -> Result<&Program, Error> {
		match self.programs.get(name) {
			Some(prog) => prog,
			None => Err(Error::new("No such program '".to_string() + name + "'")),
		}
	}

	/*
	pub fn load_buffer(&mut self, name: &str, buf: VertexBuffer) -> Result<(), Error> {
		if !self.buffers.contains_key(name) {
			self.buffers.insert(name.to_string(), buf);
			Ok(())
		} else {
			Err(Error::new("No such buffer '".to_string() + name + "'"))
		}
	}

	pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
		match self.buffers.get(name) {
			Some(buf) => Some(&buf),
			None => None,
		}
	}

	pub fn get_buffer_mut(&mut self, name: &str) -> Option<&mut Buffer> {
		match self.buffers.get_mut(name) {
			Some(ref buf) => Some(&mut buf),
			None => None,
		}
	}

	pub fn bind_camera(&self, pass: Pass) -> Pass {
		pass.uniform_matrix("proj", self.camera.proj.mat().data()).unwrap()
			.uniform_matrix("view", self.camera.model().inverse().data()).unwrap()
	}
	*/
}
