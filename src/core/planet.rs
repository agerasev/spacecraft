use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

use core::block::*;
use core::map::Map;
use core::array::Array;

pub struct Planet {
	array: Array,
	pos_: vec3d,
	ori_: mat3d,
}

impl_pos_mut!(Planet, pos_);
impl_ori_mut!(Planet, ori_);

derive_map!(Planet, array);

impl Planet {
	pub fn new(rad: i32) -> Self {
		let mut self_ = Planet { array: Array::new([rad, rad, rad].into()), pos_: vec3d::zero(), ori_: mat3d::one() };
		for iz in -rad..rad {
			for iy in -rad..rad {
				for ix in -rad..rad {
					let rv = vec3d::from([(ix as f64) + 0.5, (iy as f64) + 0.5, (iz as f64) + 0.5]);
					self_.set([ix, iy, iz].into(), if rv.length() < (rad - 1) as f64 { ROCK } else { VOID });
				}
			}
		}
		self_
	}
}