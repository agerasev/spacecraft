extern crate gl4u;

use std::rc::Rc;
use std::collections::{HashMap};

use gl4u::shader::{Shader, Type};
use gl4u::program::Program;
use gl4u::pass::Pass;

pub struct Engine {
	shaders: HashMap<String, Rc<Shader>>,
	programs: HashMap<String, Program>,
}

impl Engine {
	pub fn new() -> Self {
		Engine { shaders: HashMap::new(), programs: HashMap::new() }
	}

	pub fn load_shader(&mut self, filename: &str, sht: Type) -> Result<Rc<Shader>, String> {
		if !self.shaders.contains_key(filename) {
			let (sh, log) = try!(try!(Shader::new(sht).load_file(filename)).compile());
			if log.len() > 0 { println!("{}", log); }
			let shrc = Rc::new(sh);
			self.shaders.insert(filename.to_string(), shrc.clone());
			Ok(shrc)
		} else {
			match self.shaders.get(filename) {
			    Some(shrc) => Ok(shrc.clone()),
			    None => unreachable!(),
			}
		}
	}

	pub fn load_program(mut self, name: &str, vsfn: &str, fsfn: &str) -> Result<Self, String> {
		let vs = try!(self.load_shader(&("res/shaders/".to_string() + vsfn), Type::Vertex));
		let fs = try!(self.load_shader(&("res/shaders/".to_string() + fsfn), Type::Fragment));
		let prog = try!(Program::new().attach_rc(vs).attach_rc(fs).link());
		self.programs.insert(name.to_string(), prog);
		Ok(self)
	}

	pub fn use_program(&self, name: &str) -> Result<Pass, String> {
		match self.programs.get(name) {
			Some(prog) => prog.use_(),
			None => Err("No such program `".to_string() + name + "`"),
		}
	}
}
