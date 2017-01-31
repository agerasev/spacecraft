extern crate gl4u;

use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;
use core::planet::Planet as CorePlanet;

use view::draw::Draw;
use view::engine::Handle;
use view::model::*;

use gl4u::gl::types::*;
use gl4u::buffer::Buffer;
use gl4u::pass::Prim;

pub struct Planet {
	pub planet: CorePlanet,
	pub vertex: Buffer,
	pub color: Buffer,
}

derive_pos_mut!(Planet, planet);
derive_ori_mut!(Planet, planet);

impl_model!(Planet);

impl Planet {
	pub fn new(rad: i32) -> Self {
		let mut self_ = Planet { planet: CorePlanet::new(rad), vertex: Buffer::new(3), color: Buffer::new(3) };

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
		pass = try!(pass.uniform_matrix("model", self.model().data()));
		pass = try!(pass.attribute("position", &self.vertex));
		pass = try!(pass.attribute("color", &self.color));
		pass.primitive(Prim::Quads).range(0, 6*4).draw()
	}
}