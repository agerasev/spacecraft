use la::vec::*;

use core::block::{Block, VOID};
use core::map::Map;

pub struct Array {
	data_: Vec<Block>,
	size_: vec3i,
}

impl Array {
	pub fn new(size_: vec3i) -> Array {
		let mut v = Vec::new();
		v.resize((8*size_[0] as usize)*(size_[1] as usize)*(size_[2] as usize), VOID);
		Array { data_: v, size_: size_ }
	}

	fn itov(&self, i: usize) -> vec3i {
		vec3i::from([
			(i % (2*self.size_[0] as usize)) as i32,
			((i/(2*self.size_[0] as usize)) % (2*self.size_[1] as usize)) as i32,
			(i/((2*self.size_[0] as usize)*(2*self.size_[1] as usize))) as i32,
		])
	}

	fn vtoi(&self, v: vec3i) -> usize {
		((v[2]*2*self.size_[1] + v[1]) as usize)*2*(self.size_[0] as usize) + (v[0] as usize)
	}
}

impl Map for Array {
	fn size(&self) -> vec3i {
		self.size_
	}

	fn set(&mut self, v: vec3i, b: Block) {
		let i = self.vtoi(v);
		self.data_[i] = b;
	}

	fn get(&self, v: vec3i) -> Block {
		let i = self.vtoi(v);
		self.data_[i]
	}

	fn get_ref(&self, v: vec3i) -> &Block {
		let i = self.vtoi(v);
		&self.data_[i]
	}

	fn get_ref_mut(&mut self, v: vec3i) -> &mut Block {
		let i = self.vtoi(v);
		&mut self.data_[i]
	}
}