use la::mat::*;

pub trait Ori {
	fn ori(&self) -> mat3d;
}

pub trait OriMut {
	fn set_ori(&mut self, o: mat3d);
}