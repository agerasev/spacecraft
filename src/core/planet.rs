use num::*;
use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

use core::block::*;
use core::map::*;
use core::array::Array;
use core::chunk::*;
use core::terrain::*;

pub struct Planet {
	gen: Generator,
	pub chunks: Array<ChunkOption>,
	pub chunk_size: vec3i,
	pos_: vec3d,
	ori_: mat3d,
}

impl_pos_mut!(Planet, pos_);
impl_ori_mut!(Planet, ori_);

impl Size3 for Planet {
	fn size(&self) -> vec3i {
		self.chunks.size()*self.chunk_size*2
	}
}

impl Map<Block> for Planet {
	fn get(&self, v: vec3i) -> Block {
		if self.inside(v) {
			let (cv, cm) = v.div_mod_floor(self.chunk_size*2);
			let cm = cm - self.chunk_size;
			match *self.chunks.get_ref(cv) {
				ChunkOption::Void => VOID,
				ChunkOption::Undiscovered => ROCK,
				ChunkOption::Filled(ref chunk) => chunk.get(cm),
			}
		} else {
			VOID
		}
	}

	fn set(&mut self, v: vec3i, b: Block) {
		if self.inside(v) {
			let (cv, cm) = v.div_mod_floor(self.chunk_size*2);
			match *self.chunks.get_ref_mut(cv) {
				ChunkOption::Void => {}, // create filled chunk
				ChunkOption::Undiscovered => {}, // discover
				ChunkOption::Filled(ref mut chunk) => { chunk.set(cm, b); },
			}
		}
		self.update();
	}
}

impl Planet {
	pub fn new(chunk_size: i32, chunk_count: i32) -> Self {
		let mut p = Planet { 
			gen: Generator::new(chunk_count*2*chunk_size),
			chunks: Array::new(vec3i::from_scal(chunk_count), |_| ChunkOption::Undiscovered),
			chunk_size: vec3i::from_scal(chunk_size),
			pos_: vec3d::zero(),
			ori_: mat3d::one()
		};
		p.update();
		p
	}

	fn update(&mut self) {
		for iv in (-self.chunks.size()).iter_to(self.chunks.size()) {
			*self.chunks.get_ref_mut(iv) = self.gen.generate(iv*self.chunk_size*2 + self.chunk_size, self.chunk_size);
		}
	}
}