use la::vec::*;

use core::block::*;
use core::map::*;
use core::array::Array;

pub enum ChunkOption {
	Void,
	Undiscovered,
	Filled(Chunk),
}

pub struct Chunk {
	array: Array<Block>,
	block_count: i32,
}

derive_size3!(Chunk, array);

impl Map<Block> for Chunk {
	#[inline]
	fn get(&self, v: vec3i) -> Block {
		self.array.get(v)
	}

	fn set(&mut self, v: vec3i, b: Block) {
		let pb = self.array.get(v);
		if pb == VOID && b != VOID {
			self.block_count += 1;
		} else if pb != VOID && b == VOID {
			self.block_count -= 1;
		}
		self.array.set(v, b);
	}
}

impl Chunk {
	pub fn new<F>(size: vec3i, mut f: F) -> Chunk where F: FnMut(vec3i) -> Block {
		let mut block_count = 0;
		let array = Array::new(size, |p| {
			let b = f(p);
			if b != VOID {
				block_count += 1;
			}
			b
		});
		Chunk { array: array, block_count: block_count }
	}

	pub fn empty(&self) -> bool {
		self.block_count == 0
	}
}