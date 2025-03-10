use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod healthchecks;

use crate::healthchecks::router::health;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("========probe-api========");
    println!("Starting server at: 8080 port");
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
