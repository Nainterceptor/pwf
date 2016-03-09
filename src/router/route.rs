use router::criterion::*;
use hyper::method::{Method as hyperMethod};
use hyper::server::{Request, Response};

pub type BoxedHandler = Box<for <'a> Fn(Request, Response) + Send + Sync>;

pub struct Route {
	criteria: Vec<Criterion>,
	handler: BoxedHandler
}

impl Route {
	pub fn new(handler: BoxedHandler) -> Self {
		return Route{criteria: Vec::new(), handler: handler};
	}

	pub fn new_with_method_path(handler: BoxedHandler, method: hyperMethod, path: String) -> Self {
        let mut route = Route::new(handler);
        route.add_criterion(Criterion::Method(vec![method]));
        route.add_criterion(Criterion::ExactPath(path));
        return route;
	}

	pub fn add_criterion(&mut self, criterion: Criterion) {
		self.criteria.push(criterion);
	}

    pub fn execute(&self, req: Request, res: Response) {
        (self.handler)(req, res);
    }

	pub fn resolve(&self, request: &Request) -> bool {
		let mut bool = true;
		for criterion in self.criteria.iter() {
			if !check(criterion, request) {
				bool = false;
			}
		}
		return bool;
	}
}