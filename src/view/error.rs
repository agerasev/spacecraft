use std::error::Error as StdError;
use std::fmt::{Display, Debug, Formatter, Error as FmtError};

#[derive(Debug)]
pub struct Error {
	msg: String,
}

impl Error {
	pub fn new(msg: String) -> Error {
		Error { msg: msg }
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		&self.msg
	}
}

impl Display for Error {
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
		write!(formatter, "{}", self.description())
	}
}