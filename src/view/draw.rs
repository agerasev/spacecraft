extern crate gl4u;

use gl4u::pass::Pass;

use view::engine::Engine;

pub trait Draw {
	fn draw(&self, engine: &Engine) -> Pass;
}