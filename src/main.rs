mod plexer;
use plexer::{Plexer, Route};
use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Method, Request, Response, Server, StatusCode, Uri,
    },
    std::net::SocketAddr,
};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    if let Err(e) = serve_future.await {
        eprintln!("Server error: {}", e);
    }
}

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got a request at {:?}", req.uri());

    let mut plexer = Plexer::new();
    plexer.register(Route::new(
        String::from("/hello"),
        &Method::GET,
        |_r: Request<Body>| Ok(Response::new(Body::from("hello world"))),
    ));

    plexer.dispatch(req)
}
