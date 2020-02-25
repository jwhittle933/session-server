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
    pub fn dispatch() {}
}

#[derive(Debug)]
struct Route {
    path: String,
    methods: Vec<Method>,
    // handler: service_fn
}
