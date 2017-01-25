extern crate gl4u;

use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;
use core::planet::Planet as CorePlanet;

use view::draw::Draw;
use view::engine::Engine;

use gl4u::gl::types::*;
use gl4u::buffer::Buffer;
use gl4u::pass::{Pass, Prim};

pub struct Planet {
	pub planet: CorePlanet,
	pub vertex: Buffer,
	pub color: Buffer,
	pub model: mat4<GLfloat>
}

impl Planet {
	pub fn new() -> Self {
		let mut self_ = Planet { planet: CorePlanet::new(), vertex: Buffer::new(3), color: Buffer::new(3), model: mat4::one() };
		self_.set_pos(vec3d::zero());
		self_.set_ori(mat3d::one());

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

impl Pos for Planet {
	fn pos(&self) -> vec3d {
		self.planet.pos()
	}
}

impl PosMut for Planet {
	fn set_pos(&mut self, p: vec3d) {
		self.planet.set_pos(p);
		for i in 0..3 {
			self.model[(3, i)] = p[i] as GLfloat;
		}
	}
}

impl Ori for Planet {
	fn ori(&self) -> mat3d {
		self.planet.ori()
	}
}

impl OriMut for Planet {
	fn set_ori(&mut self, o: mat3d) {
		self.planet.set_ori(o);
		for j in 0..3 {
			for i in 0..3 {
				self.model[(i, j)] = o[(i, j)] as GLfloat;
			}
		}
	}
}

impl Draw for Planet {
	fn draw(&self, engine: &Engine) -> Pass {
		engine.use_program("main").unwrap()
			.uniform_matrix("model", self.model.data()).unwrap()
			.attribute("position", &self.vertex).unwrap()
			.attribute("color", &self.color).unwrap()
			.primitive(Prim::Quads).range(0, 6*4)
	}
}