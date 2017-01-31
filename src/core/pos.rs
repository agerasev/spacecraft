use la::vec::*;

pub trait Pos {
	fn pos(&self) -> vec3d;
}

pub trait PosMut: Pos {
	fn set_pos(&mut self, p: vec3d);
}

macro_rules! impl_pos {
	($Struct:ident, $field:ident) => (
		impl Pos for $Struct {
			fn pos(&self) -> vec3d {
				self.$field
			}
		}
	)
}

macro_rules! impl_pos_mut {
	($Struct:ident, $field:ident) => (
		impl_pos!($Struct, $field);

		impl PosMut for $Struct {
			fn set_pos(&mut self, p: vec3d) {
				self.$field = p;
			}
		}
	)
}

macro_rules! derive_pos {
	($Struct:ident, $field:ident) => (
		impl Pos for $Struct {
			fn pos(&self) -> vec3d {
				self.$field.pos()
			}
		}
	)
}

macro_rules! derive_pos_mut {
	($Struct:ident, $field:ident) => (
		derive_pos!($Struct, $field);

		impl PosMut for $Struct {
			fn set_pos(&mut self, p: vec3d) {
				self.$field.set_pos(p);
			}
		}
	)
}