use actix_web::{get, web, App, HttpServer, Responder};
use actix_files as fs;

mod functions;

/// The endpoint to greet a user
#[get("/hello/{name}")]
async fn greet_req(name: web::Path<String>) -> impl Responder {
    functions::greet(name.to_string())
}

/// The main function defining:
/// * the Actix HttpServer
/// * register `/hello` for a hello world message
/// * register `/hello/{name}` for greeting a user
/// * register static file serving for ./content at `/static`
/// * binding to port `8080` on the current machine
///
/// ### Registering a new service
///
/// * Copy the `greet_req` endpoint and
/// abstract code into the `functions` crate.
/// * choose HTTP Method (there is more than GET)
/// * alter route and parameters
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
