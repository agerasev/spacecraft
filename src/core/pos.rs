use la::vec::*;

pub trait Pos {
	fn pos(&self) -> vec3f;
}

pub trait PosMut: Pos {
	fn set_pos(&mut self, p: vec3f);
}

macro_rules! impl_pos {
	($Struct:ident, $field:ident) => (
		impl Pos for $Struct {
			fn pos(&self) -> vec3f {
				self.$field
			}
		}
	)
}

macro_rules! impl_pos_mut {
	($Struct:ident, $field:ident) => (
		impl_pos!($Struct, $field);

		impl PosMut for $Struct {
			fn set_pos(&mut self, p: vec3f) {
				self.$field = p;
			}
		}
	)
}

macro_rules! derive_pos {
	($Struct:ident, $field:ident) => (
		impl Pos for $Struct {
			fn pos(&self) -> vec3f {
				self.$field.pos()
			}
		}
	)
}

macro_rules! derive_pos_mut {
	($Struct:ident, $field:ident) => (
		derive_pos!($Struct, $field);

		impl PosMut for $Struct {
			fn set_pos(&mut self, p: vec3f) {
				self.$field.set_pos(p);
			}
		}
	)
}