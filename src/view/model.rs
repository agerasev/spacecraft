use la::mat::*;

use core::pos::*;
use core::ori::*;

pub trait Model: Pos + Ori {
	fn model(&self) -> mat4f;
}

macro_rules! impl_model {
	($Struct:ident) => (
		impl Model for $Struct {
			fn model(&self) -> mat4f {
				let mut m: mat4f = mat4::one();
				let (p, o) = (self.pos(), self.ori());
				for j in 0..3 {
					for i in 0..3 {
						m[(i, j)] = o[(i, j)];
					}
					m[(3, j)] = p[j];
				}
				m
			}
		}
	)
}

macro_rules! derive_model {
	($Struct:ident, $field:ident) => (
		impl Model for $Struct {
			#[inline]
			fn model(&self) -> mat4f {
				self.$field.model()
			}
		}
	)
}