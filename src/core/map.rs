use std::ops::{Index, IndexMut};

use la::vec::*;

use core::block::Block;

pub trait Map {
	fn size(&self) -> vec3i; 

	fn get(&self, v: vec3i) -> Block;
	fn set(&mut self, v: vec3i, b: Block);

	fn get_ref(&self, v: vec3i) -> &Block;
	fn get_ref_mut(&mut self, v: vec3i) -> &mut Block;
}

impl Index<vec3i> for Map {
	type Output = Block;
	fn index(&self, p: vec3i) -> &Block {
		self.get_ref(p)
	}
}

impl IndexMut<vec3i> for Map {
	fn index_mut(&mut self, p: vec3i) -> &mut Block {
		self.get_ref_mut(p)
	}
}

macro_rules! derive_map {
	($Struct:ident, $field:ident) => (
		impl Map for $Struct {
			fn size(&self) -> vec3i {
				self.$field.size()
			}

			fn set(&mut self, v: vec3i, b: Block) {
				self.$field.set(v, b);
			}

			fn get(&self, v: vec3i) -> Block {
				self.$field.get(v)
			}

			fn get_ref(&self, v: vec3i) -> &Block {
				self.$field.get_ref(v)
			}

			fn get_ref_mut(&mut self, v: vec3i) -> &mut Block {
				self.$field.get_ref_mut(v)
			}
		}
	)
}
