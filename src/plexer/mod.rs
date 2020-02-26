use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, StatusCode, Uri};

type HandleFn = fn(Request<Body>) -> Result<Response<Body>, hyper::Error>;

#[derive(Debug)]
pub struct Plexer<'a> {
    routes: Vec<Route<'a>>,
}

impl Plexer<'_> {
    pub fn new() -> Plexer<'static> {
        Plexer {
            routes: Vec::<Route<'_>>::new(),
        }
    }
    pub fn register(&mut self, r: Route<'static>) {
        self.routes.push(r);
    }
    pub fn dispatch(&self, _r: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        Ok(Response::new(Body::from("hello world")))
    }
}

trait Handle<'a> {
    fn new(path: String, methods: Vec<&'a Method>, hf: HandleFn) -> Self;
    fn handle(&self, r: Request<Body>) -> Result<Response<Body>, hyper::Error>;
}

#[derive(Debug)]
pub struct Route<'a> {
    path: String,
    methods: &'a Method,
    handler: HandleFn,
}

impl<'a> Route<'a> {
    pub fn new(path: String, method: &'a Method, hf: HandleFn) -> Route {
        Route {
            path: path,
            methods: method,
            handler: hf,
        }
    }
    pub fn handle(&self, r: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        (self.handler)(r)
    }
}
