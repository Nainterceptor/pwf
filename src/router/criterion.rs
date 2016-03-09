extern crate hyper;
use hyper::method::Method;
use hyper::server::{Request};

#[derive(Clone)]
pub enum Criterion {
	Any,
	Domain(String),
	ExactPath(String),
	Method(Vec<Method>),
	PathStartWith(String),
}

pub fn is_extension(method: &Method) -> bool {
    return match method {
        &Method::Extension(ref x) => true,
        _ => false
    };
}

pub fn check(criterion: &Criterion, request: &Request) -> bool {
	return match *criterion {
		Criterion::Any => true,
		Criterion::Domain(ref domain) => true,
		Criterion::ExactPath(ref path) => request.uri.to_string().eq(path),
		Criterion::Method(ref methods) => {
            for method in methods.iter() {
                if request.method.eq(method) || (is_extension(&request.method) && request.method.eq(&Method::Extension(String::new()))){
                    return true;
                }
            }
            return false;
        },
		Criterion::PathStartWith(ref path) => request.uri.to_string().starts_with(path),
	}
}

