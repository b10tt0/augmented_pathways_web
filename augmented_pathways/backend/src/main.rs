use actix_web::{App, HttpServer};
use actix_web_lab::web::spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/augmented-paths")
                .static_resources_location("./dist")
                .finish(),
        )
    })
    .workers(4)
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
