use la::vec::*;

use core::block::{Block, VOID};
use core::map::Map;

pub struct Array {
	data_: Vec<Block>,
	size_: vec3i,
}

impl Array {
	pub fn new(size: vec3i) -> Array {
		assert!(size.gt_(vec3::zero()).all());
		let mut v = Vec::new();
		v.resize((8*size[0] as usize)*(size[1] as usize)*(size[2] as usize), VOID);
		Array { data_: v, size_: size }
	}

	fn itov(&self, i: usize) -> vec3i {
		let s = self.size_;
		vec3i::from([
			(i % (2*s[0] as usize)) as i32 - s[0],
			((i/(2*s[0] as usize)) % (2*s[1] as usize)) as i32 - s[1],
			(i/((2*s[0] as usize)*(2*s[1] as usize))) as i32 - s[2],
		])
	}

	fn vtoi(&self, v: vec3i) -> usize {
		let s = self.size_;
		(((v[2] + s[2])*(2*s[1]) + (v[1] + s[1])) as usize)*(2*s[0] as usize) + (v[0] + s[0]) as usize
	}

	#[inline]
	fn check_bounds(&self, v: vec3i) {
		assert!((v.ge_(-self.size_) & v.lt_(self.size_)).all());
	}
}

impl Map for Array {
	fn size(&self) -> vec3i {
		self.size_
	}

	fn set(&mut self, v: vec3i, b: Block) {
		self.check_bounds(v);
		let i = self.vtoi(v);
		self.data_[i] = b;
	}

	fn get(&self, v: vec3i) -> Block {
		self.check_bounds(v);
		let i = self.vtoi(v);
		self.data_[i]
	}
}