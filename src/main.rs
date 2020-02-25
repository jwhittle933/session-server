mod plexer;
use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Method, Request, Response, Server, StatusCode, Uri,
    },
    std::net::SocketAddr,
    tokio::net::TcpListener,
    tokio::prelude::*,
};

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got a request at {:?}", req.uri());
    Ok(Response::new(Body::from("hello world")))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr).await;
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    if let Err(e) = serve_future.await {
        eprintln!("Server error: {}", e);
    }
}

async fn alt_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
