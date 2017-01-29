extern crate gl4u;

use view::draw::Draw;
use view::engine::Handle;
use view::model::*;

use gl4u::buffer::Buffer;
use gl4u::pass::Prim;

pub struct Planet {
	pub model: Model,
	pub vertex: Buffer,
	pub color: Buffer,
}

impl Planet {
	pub fn new() -> Self {
		let mut self_ = Planet { vertex: Buffer::new(3), color: Buffer::new(3), model: Model::new() };

		self_.vertex.load_float(&[
			 1.0, 1.0, 1.0,
			 1.0,-1.0, 1.0,
			 1.0,-1.0,-1.0,
			 1.0, 1.0,-1.0,

			-1.0, 1.0, 1.0,
			 1.0, 1.0, 1.0,
			 1.0, 1.0,-1.0,
			-1.0, 1.0,-1.0,

			-1.0,-1.0, 1.0,
			-1.0, 1.0, 1.0,
			-1.0, 1.0,-1.0,
			-1.0,-1.0,-1.0,

			 1.0,-1.0, 1.0,
			-1.0,-1.0, 1.0,
			-1.0,-1.0,-1.0,
			 1.0,-1.0,-1.0,

			 1.0, 1.0, 1.0,
			-1.0, 1.0, 1.0,
			-1.0,-1.0, 1.0,
			 1.0,-1.0, 1.0,

			 1.0,-1.0,-1.0,
			-1.0,-1.0,-1.0,
			-1.0, 1.0,-1.0,
			 1.0, 1.0,-1.0,
		]);

		self_.color.load_float(&[
			1.0, 0.0, 0.0,
			1.0, 0.0, 0.0,
			1.0, 0.0, 0.0,
			1.0, 0.0, 0.0,

			0.0, 1.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 1.0, 0.0,

			0.0, 1.0, 1.0,
			0.0, 1.0, 1.0,
			0.0, 1.0, 1.0,
			0.0, 1.0, 1.0,

			1.0, 0.0, 1.0,
			1.0, 0.0, 1.0,
			1.0, 0.0, 1.0,
			1.0, 0.0, 1.0,

			1.0, 1.0, 0.0,
			1.0, 1.0, 0.0,
			1.0, 1.0, 0.0,
			1.0, 1.0, 0.0,

			0.0, 0.0, 1.0,
			0.0, 0.0, 1.0,
			0.0, 0.0, 1.0,
			0.0, 0.0, 1.0,
		]);

		self_
	}
}

impl Draw for Planet {
	fn draw(&self, handle: &Handle) -> Result<(), String> {
		let mut pass = handle.use_program("main");
		pass = try!(pass.uniform_matrix("model", self.model.mat().data()));
		pass = try!(pass.attribute("position", &self.vertex));
		pass = try!(pass.attribute("color", &self.color));
		pass.primitive(Prim::Quads).range(0, 6*4).draw()
	}
}