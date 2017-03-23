use la::mat::*;

pub trait Ori {
	fn ori(&self) -> mat3f;
}

pub trait OriMut {
	fn set_ori(&mut self, o: mat3f);
}

macro_rules! impl_ori {
	($Struct:ident, $field:ident) => (
		impl Ori for $Struct {
			fn ori(&self) -> mat3f {
				self.$field
			}
		}
	)
}

macro_rules! impl_ori_mut {
	($Struct:ident, $field:ident) => (
		impl_ori!($Struct, $field);

		impl OriMut for $Struct {
			fn set_ori(&mut self, o: mat3f) {
				self.$field = o;
			}
		}
	)
}

macro_rules! derive_ori {
	($Struct:ident, $field:ident) => (
		impl Ori for $Struct {
			#[inline]
			fn ori(&self) -> mat3f {
				self.$field.ori()
			}
		}
	)
}

macro_rules! derive_ori_mut {
	($Struct:ident, $field:ident) => (
		derive_ori!($Struct, $field);

		impl OriMut for $Struct {
			#[inline]
			fn set_ori(&mut self, o: mat3f) {
				self.$field.set_ori(o);
			}
		}
	)
}