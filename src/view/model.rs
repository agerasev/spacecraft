extern crate gl4u;

use la::mat::*;

use core::pos::*;
use core::ori::*;

use gl4u::gl::types::*;

pub trait Model: Pos + Ori {
	fn model(&self) -> mat4<GLfloat>;
}

macro_rules! impl_model {
	($Struct:ident) => (
		impl Model for $Struct {
			fn model(&self) -> mat4<GLfloat> {
				let mut m: mat4<GLfloat> = mat4::one();
				let (p, o) = (self.pos(), self.ori());
				for j in 0..3 {
					for i in 0..3 {
						m[(i, j)] = o[(i, j)] as GLfloat;
					}
					m[(3, j)] = p[j] as GLfloat;
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
			fn model(&self) -> mat4<GLfloat> {
				self.$field.model()
			}
		}
	)
}