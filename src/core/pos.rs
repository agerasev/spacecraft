use la::vec::*;

pub trait Pos {
	fn pos(&self) -> vec3d;
}

pub trait PosMut: Pos {
	fn set_pos(&mut self, p: vec3d);
}

