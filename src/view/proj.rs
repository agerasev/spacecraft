use num::*;
use la::mat::*;

pub struct Proj {
	pub f: f32,
	pub n: f32,
	pub w: f32,
	pub h: f32,
	pub mat_: mat4f,
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

	pub fn mat(&self) -> mat4f {
		self.mat_
	}

	pub fn set_fn(&mut self, f: f32, n: f32) {
		self.f = f;
		self.n = n;
		self.update();
	}

	pub fn set_wh(&mut self, w: f32, h: f32) {
		self.w = w;
		self.h = h;
		self.update();
	}

	fn update(&mut self) {
		self.mat_[(0, 0)] = self.n/self.w;
		self.mat_[(1, 1)] = self.n/self.h;
		self.mat_[(2, 2)] = -(self.f + self.n)/(self.f - self.n);
		self.mat_[(3, 2)] = -2.0*self.f*self.n/(self.f - self.n);
		self.mat_[(2, 3)] = -1.0;
		self.mat_[(3, 3)] = 0.0;
	}
}
