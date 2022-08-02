use actix_web::{get, web, App, HttpServer, Responder};

mod functions;

#[get("/hello/{name}")]
async fn greet_req(name: web::Path<String>) -> impl Responder {
    functions::greet(name.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet_req)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
