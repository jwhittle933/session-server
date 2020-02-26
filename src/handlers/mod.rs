mod handlers;
use hyper::{Body, Request, Response, StatusCode, Uri};

type HandleFn = fn(Request<Body>) -> Result<Response<Body>, hyper::Error>;
