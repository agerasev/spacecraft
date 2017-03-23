use la::vec::*;

use core::block::*;
use core::chunk::*;

pub struct Generator {
	rad: i32,
}

impl Generator {
	pub fn new(rad: i32) -> Generator {
		Generator { rad: rad }
	}

	pub fn generate(&self, center: vec3i, chunk_size: vec3i) -> ChunkOption {
		let sea_level = (3*self.rad)/4;
		ChunkOption::Filled(Chunk::new(chunk_size, |rel| {
			let pos = center + rel;
			// pos.max_() < sea_level && pos.min_() >= -sea_level
			if (pos.as_::<vec3f>() + vec3f::from_scal(0.5)).length() <= sea_level as f32 {
				ROCK
			} else {
				VOID
			}
		}))
	}
}