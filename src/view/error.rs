use std::error::Error as StdError;
use std::fmt;

use gl4u::error::Error as GlError;

pub enum Error {
	Msg { msg: String },
	Gl { err: GlError },
}

impl Error {
	pub fn new(msg: String) -> Error {
		Error::Msg { msg: msg }
	}

	pub fn from_gl(err: GlError) -> Error {
		Error::Gl { err: GlError }
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match self {
			Error::Msg(msg) => &msg,
			Error::Gl(err) => err.description(),
		}
	}

	fn cause(&self) -> Option<&StdError> {
		match self {
			Error::Msg => None,
			Error::Gl(err) => Some(&err),
		}
	}
}
