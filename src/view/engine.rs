use std::rc::Rc;
use std::collections::{HashMap};

use gl4u::shader::{Shader, Type};
use gl4u::program::Program;
use gl4u::buffer::Buffer;
use gl4u::pass::Pass;
use gl4u::error::Error;

use view::model::Model;
use view::camera::Camera;

pub struct Engine {
	shaders: HashMap<String, Rc<Shader>>,
	programs: HashMap<String, Program>,
	buffers: HashMap<String, Buffer>,
	pub camera: Camera,
}

pub struct Handle<'a> {
	engine: &'a Engine,
}

impl Engine {
	pub fn new() -> Self {
		Engine { shaders: HashMap::new(), programs: HashMap::new(), buffers: HashMap::new(), camera: Camera::new() }
	}

	pub fn load_shader(&mut self, filename: &str, sht: Type) -> Result<Rc<Shader>, Error> {
		if !self.shaders.contains_key(filename) {
			match try!(Shader::new(sht).load_file(filename)).compile() {
				Ok((sh, log)) => {
					if log.len() > 0 { println!("{}", log); }
					let shrc = Rc::new(sh);
					self.shaders.insert(filename.to_string(), shrc.clone());
					Ok(shrc)
				},
				Err((err, log)) => {
					println!("{}", log);
					Err(err)
				},
			}
		} else {
			match self.shaders.get(filename) {
			    Some(shrc) => Ok(shrc.clone()),
			    None => unreachable!(),
			}
		}
	}

	pub fn load_program(&mut self, name: &str, vsfn: &str, fsfn: &str) -> Result<(), Error> {
		let vs = try!(self.load_shader(&("res/shaders/".to_string() + vsfn), Type::Vertex));
		let fs = try!(self.load_shader(&("res/shaders/".to_string() + fsfn), Type::Fragment));
		let prog = try!(Program::new().attach_rc(vs).attach_rc(fs).link());
		self.programs.insert(name.to_string(), prog);
		Ok(())
	}

	pub fn use_program(&self, name: &str) -> Result<Pass, Error> {
		match self.programs.get(name) {
			Some(prog) => prog.use_(),
			None => Err(Error::new("No such program `".to_string() + name + "`")),
		}
	}

	pub fn load_buffer(&mut self, name: &str, buf: Buffer) -> Result<(), Error> {
		if !self.buffers.contains_key(name) {
			self.buffers.insert(name.to_string(), buf);
			Ok(())
		} else {
			Err(Error::new("No such buffer `".to_string() + name + "`"))
		}
	}

	/*
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
	*/

	pub fn bind_camera(&self, pass: Pass) -> Pass {
		pass.uniform_matrix("proj", self.camera.proj.mat().data()).unwrap()
		    .uniform_matrix("view", self.camera.model().inverse().data()).unwrap()
	}
}
