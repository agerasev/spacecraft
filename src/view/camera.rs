extern crate gl4u;

use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

use gl4u::gl::types::*;

struct Camera {
	pub p: vec3d,
	pub o: mat3d,

	pub f: f64,
	pub n: f64,
	pub w: f64,
	pub h: f64,

	pub view: mat4<GLfloat>,
	pub proj: mat4<GLfloat>,
}

impl Camera {
	pub fn new() -> Self {
		let (f, n) = (1e2, 1e-2);
		Camera {
			p: vec3d::zero(),
			o: mat3d::one(),
			f: f,
			n: n,
			w: n,
			h: n,
			view: mat4::one(),
			proj: mat4::one(),
		}
	}

	pub fn update_proj(&mut self) {
		self.proj[(0, 0)] = (self.n/self.w) as GLfloat;
		self.proj[(1, 1)] = (self.n/self.h) as GLfloat;
		self.proj[(2, 2)] = (-(self.f + self.n)/(self.f - self.n)) as GLfloat;
		self.proj[(3, 2)] = (-2.0*self.f*self.n/(self.f - self.n)) as GLfloat;
		self.proj[(2, 3)] = -1.0;
		self.proj[(3, 3)] = 0.0;
	}
}

impl Pos for Camera {
	fn pos(&self) -> vec3d {
		self.p
	}
}

impl PosMut for Camera {
	fn set_pos(&mut self, p: vec3d) {
		self.p = p;
	}
}

impl Ori for Camera {
	fn ori(&self) -> mat3d {
		self.o
	}
}

impl OriMut for Camera {
	fn set_ori(&mut self, o: mat3d) {
		self.o = o
	}
}