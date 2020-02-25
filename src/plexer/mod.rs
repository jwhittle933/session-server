use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, StatusCode, Uri};

#[derive(Debug)]
struct Plexer<'a> {
    routes: Vec<&'a Route>,
}

impl Plexer<'_> {
    pub fn new() -> Plexer<'static> {
        Plexer { routes: Vec::new() }
    }
    pub fn register(&mut self, r: &'static Route) {
        self.routes.push(r);
    }
    pub fn dispatch(&self) {
        let service =
            make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(place_holder)) });
    }
}

async fn place_holder(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Yup")))
}

#[derive(Debug)]
struct Route<F: Fn(Request<Body>) -> Result<Response<Body>, hyper::Error>> {
    path: String,
    methods: Vec<Method>,
    handler: F,
}

impl<F: Fn(Request<Body>) -> Result<Response<Body>, hyper::Error>> Route<F> {
    pub fn handler(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        (self.handler)(req)
    }
}
