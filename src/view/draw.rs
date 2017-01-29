extern crate gl4u;

use view::engine::Handle;

pub trait Draw {
	fn draw(&self, handle: &Handle) -> Result<(), String>;
}