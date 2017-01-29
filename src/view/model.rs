extern crate gl4u;

use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

use gl4u::gl::types::*;

pub struct Model {
	pos_: vec3d,
	ori_: mat3d,
	mat_: mat4<GLfloat>,
}

impl Model {
	pub fn new() -> Self {
		Model { pos_: vec3::zero(), ori_: mat3::zero(), mat_: mat4::one() }
	}

	pub fn mat(&self) -> mat4<GLfloat> {
		self.mat_
	}
}

impl Pos for Model {
	fn pos(&self) -> vec3d {
		self.pos_
	}
}

impl PosMut for Model {
	fn set_pos(&mut self, pos: vec3d) {
		self.pos_ = pos;
		for i in 0..3 {
			self.mat_[(3, i)] = pos[i] as GLfloat;
		}
	}
}

impl Ori for Model {
	fn ori(&self) -> mat3d {
		self.ori_
	}
}

impl OriMut for Model {
	fn set_ori(&mut self, ori: mat3d) {
		self.ori_ = ori;
		for j in 0..3 {
			for i in 0..3 {
				self.mat_[(i, j)] = ori[(i, j)] as GLfloat;
			}
		}
	}
}
