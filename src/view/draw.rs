extern crate gl4u;

use view::engine::Handle;

pub trait Draw {
	fn draw(&mut self, handle: &Handle) -> Result<(), String>;
}