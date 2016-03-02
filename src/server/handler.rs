use hyper::server::{Request, Response};

use router::{Container};
use router::route::{Route};

use hyper::server::Handler;

pub struct ServerHandler {
    router: Container,
}

impl Handler for ServerHandler {
    fn handle(&self, req: Request, res: Response) {
        let route: &Route = self.router.resolve(&req).unwrap();
        route.execute(req, res);
    }
}

impl ServerHandler {
    pub fn new(router: Container) -> Self {
        return ServerHandler { router: router }
    }
}
