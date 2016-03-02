pub mod criterion;
pub mod route;
pub mod error;

use router::route::{Route};
use router::error::{RouterError};
use hyper::server::{Request};

pub struct Container {
	routes: Vec<Route>
}

impl Container {
	pub fn new() -> Self {
		return Container{ routes: Vec::new() };
	}
	pub fn add_route(&mut self, route: Route) {
			self.routes.push(route);
	}
	pub fn resolve(&self, request: &Request) -> Result<&Route, RouterError>{
		for route in self.routes.iter() {
			if route.resolve(request) {
				return Ok(route);
			}
		}
		return Err(RouterError::RouteNotFound);
	}
}