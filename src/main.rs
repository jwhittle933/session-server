use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/mt")
                    .guard(guard::Header("Host", "www.servetext.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/lxx")
                    .guard(guard::Header("Host", "www.servetext.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}
