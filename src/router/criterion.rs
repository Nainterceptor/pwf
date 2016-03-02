extern crate hyper;
use hyper::method::{Method as hyperMethod};
use hyper::server::{Request};

#[derive(Clone)]
pub enum Criterion {
	Any,
	Domain(String),
	ExactPath(String),
	Method(hyperMethod),
}

pub fn check(criterion: &Criterion, request: &Request) -> bool {
	return match *criterion {
		Criterion::Any => true,
		Criterion::Domain(ref domain) => true,
		Criterion::ExactPath(ref path) => request.uri.to_string().eq(path),
		Criterion::Method(ref method) => request.method.eq(method),
	}
}

