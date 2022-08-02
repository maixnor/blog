use actix_web::{get, web, App, HttpServer, Responder};
use actix_files as fs;

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
            .service(fs::Files::new("/static", "./content").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
