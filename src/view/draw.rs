use view::engine::Engine;
use gl4u::error::Error;

pub trait Draw {
	fn draw(&mut self, engine: &mut Engine) -> Result<(), Error>;
}