extern crate gl4u;

use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

use gl4u::gl::types::*;

pub trait Model : Pos + Ori {
	fn model(&self) -> mat4<GLfloat>;
}

pub trait ModelMut : Model + PosMut + OriMut {

}