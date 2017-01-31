use la::vec::*;

use core::block::Block;

pub trait Map {
	fn size(&self) -> vec3i; 

	fn get(&self, v: vec3i) -> Block;
	fn set(&mut self, v: vec3i, b: Block);
}

macro_rules! derive_map {
	($Struct:ident, $field:ident) => (
		impl Map for $Struct {
			#[inline]
			fn size(&self) -> vec3i {
				self.$field.size()
			}

			#[inline]
			fn set(&mut self, v: vec3i, b: Block) {
				self.$field.set(v, b);
			}

			#[inline]
			fn get(&self, v: vec3i) -> Block {
				self.$field.get(v)
			}
		}
	)
}
