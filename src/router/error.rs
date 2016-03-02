use std::error;
use std::fmt;

#[derive(Debug)]
pub enum RouterError {
	RouteNotFound,
}

impl fmt::Display for RouterError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
				RouterError::RouteNotFound => write!(f, "Route not found in container"),
		}
	}
}

impl error::Error for RouterError {
	fn description(&self) -> &str {
		return match *self {
				RouterError::RouteNotFound => "Route not found in container",
		};
	}

	fn cause(&self) -> Option<&error::Error> {
		return match *self {
				RouterError::RouteNotFound => None,
		};
	}
}