use actix_web::{App, HttpServer};
use actix_web_lab::web::spa;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
