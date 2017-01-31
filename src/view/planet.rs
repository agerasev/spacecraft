extern crate gl4u;

use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;
use core::block::Block;
use core::map::Map as CoreMap;
use core::planet::Planet as CorePlanet;

use view::draw::Draw;
use view::engine::Handle;
use view::model::*;

use gl4u::gl::types::*;
use gl4u::buffer::Buffer;
use gl4u::pass::Prim;

pub struct Planet {
	planet: CorePlanet,
	vertex: Buffer,
	color: Buffer,
	dirty: bool,
}

derive_pos_mut!(Planet, planet);
derive_ori_mut!(Planet, planet);

impl_model!(Planet);

impl CoreMap for Planet {
	#[inline]
	fn size(&self) -> vec3i {
		self.planet.size()
	}
	
	#[inline]
	fn get(&self, v: vec3i) -> Block {
		self.planet.get(v)
	}

	#[inline]
	fn set(&mut self, v: vec3i, b: Block) {
		self.planet.set(v, b);
		self.dirty = true;
	}
}

impl Draw for Planet {
	fn draw(&mut self, handle: &Handle) -> Result<(), String> {
		if self.dirty { self.update(); }
		let mut pass = handle.use_program("main");
		pass = try!(pass.uniform_matrix("model", self.model().data()));
		pass = try!(pass.attribute("position", &self.vertex));
		pass = try!(pass.attribute("color", &self.color));
		pass.primitive(Prim::Quads).range(0, 6*4).draw()
	}
}

impl Planet {
	pub fn new(rad: i32) -> Self {
		Planet { planet: CorePlanet::new(rad), vertex: Buffer::new(3), color: Buffer::new(3), dirty: true }
	}

	pub fn update(&mut self) {
		self.vertex.load_float(&[
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

		self.color.load_float(&[
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

		self.dirty = false;
	}
}
