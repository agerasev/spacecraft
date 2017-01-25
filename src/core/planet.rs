use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

pub struct Planet {
	pub p: vec3d,
	pub o: mat3d
}

impl Planet {
	pub fn new() -> Self {
		Planet { p: vec3d::zero(), o: mat3d::one() }
	}
}

impl Pos for Planet {
	fn pos(&self) -> vec3d {
		self.p
	}
}

impl PosMut for Planet {
	fn set_pos(&mut self, p: vec3d) {
		self.p = p;
	}
}

impl Ori for Planet {
	fn ori(&self) -> mat3d {
		self.o
	}
}

impl OriMut for Planet {
	fn set_ori(&mut self, o: mat3d) {
		self.o = o;
	}
}
