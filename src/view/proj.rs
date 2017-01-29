extern crate gl4u;

use la::mat::*;
use gl4u::gl::types::*;

pub struct Proj {
	pub f: f64,
	pub n: f64,
	pub w: f64,
	pub h: f64,
	pub mat_: mat4<GLfloat>,
}

impl Proj {
	pub fn new() -> Self {
		let (f, n) = (1e2, 1e-2);
		let mut self_ = Proj {
			f: f,
			n: n,
			w: n,
			h: n,
			mat_: mat4::one(),
		};
		self_.update();
		self_
	}

	pub fn mat(&self) -> mat4<GLfloat> {
		self.mat_
	}

	pub fn set_fn(&mut self, f: f64, n: f64) {
		self.f = f;
		self.n = n;
		self.update();
	}

	pub fn set_wh(&mut self, w: f64, h: f64) {
		self.w = w;
		self.h = h;
		self.update();
	}

	fn update(&mut self) {
		self.mat_[(0, 0)] = (self.n/self.w) as GLfloat;
		self.mat_[(1, 1)] = (self.n/self.h) as GLfloat;
		self.mat_[(2, 2)] = (-(self.f + self.n)/(self.f - self.n)) as GLfloat;
		self.mat_[(3, 2)] = (-2.0*self.f*self.n/(self.f - self.n)) as GLfloat;
		self.mat_[(2, 3)] = -1.0;
		self.mat_[(3, 3)] = 0.0;
	}
}
