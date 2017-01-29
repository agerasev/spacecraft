extern crate gl4u;

use la::vec::*;
use la::mat::*;

use view::model::*;
use view::proj::*;

pub struct Camera {
	pub p: vec3d,
	pub o: mat3d,

	pub model: Model,
	pub proj: Proj,
}

impl Camera {
	pub fn new() -> Self {
		Camera {
			p: vec3d::zero(),
			o: mat3d::one(),
			model: Model::new(),
			proj: Proj::new(),
		}
	}
}
