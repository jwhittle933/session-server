use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, StatusCode, Uri};

type HandleFn = fn(Request<Body>) -> Result<Response<Body>, hyper::Error>;

#[derive(Debug)]
struct Plexer<'a> {
    routes: Vec<&'a Route>,
}

impl Plexer<'_> {
    pub fn new() -> Plexer<'static> {
        Plexer {
            routes: Vec::<&Route>::with_capacity(1),
        }
    }
    pub fn register(&mut self, r: &'static Route) {
        self.routes.push(r);
    }
    pub fn dispatch(&self) {}
}

trait Handle {
    fn new(path: String, hf: HandleFn) -> Self;
    fn handle(&self, r: Request<Body>) -> Result<Response<Body>, hyper::Error>;
}

#[derive(Debug)]
struct Route {
    path: String,
    methods: Vec<Method>,
    handler: HandleFn,
}

impl Handle for Route {
    fn new(path: String, hf: HandleFn) -> Route {
        Route {
            path: path,
            methods: vec![],
            handler: hf,
        }
    }
    fn handle(&self, r: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        (self.handler)(r)
    }
}
