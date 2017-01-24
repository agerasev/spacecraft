extern crate gl4u;

use std::collections::{HashMap};

use gl4u::shader::{Shader, Type};
use gl4u::program::Program;

struct ProgramExt {
	program: Program,
	vsfn: String,
	fsfn: String,
}

pub struct Engine {
	shaders: HashMap<String, Shader>,
	programs: HashMap<String, ProgramExt>,
}

impl Engine {
	pub fn new() -> Self {
		Engine { shaders: HashMap::new(), programs: HashMap::new() }
	}

	fn load_shader(&mut self, filename: &str, sht: Type) -> Result<(), String> {
		if !self.shaders.contains_key(filename) {
			let (sh, log) = try!(try!(Shader::new(sht).load_file(filename)).compile());
			if log.len() > 0 { println!("{}", log); }
			self.shaders.insert(filename.to_string(), sh);
		}
		Ok(())
	}

	pub fn load_program(mut self, name: &str, vsfn: &str, fsfn: &str) -> Result<Self, String> {
		try!(self.load_shader(vsfn, Type::Vertex));
		try!(self.load_shader(fsfn, Type::Fragment));
		self.programs.insert(name.to_string(), ProgramExt { program: Program::new(), vsfn: vsfn.to_string(), fsfn: fsfn.to_string() });
		Ok(self)
	}
}
