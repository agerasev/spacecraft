extern crate gl4u;

use gl4u::pass::Pass;

use view::engine::Handle;

pub trait Draw {
	fn draw(&self, handle: &Handle) -> Pass;
}