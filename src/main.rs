use actix_web::{get, web::Data, App, HttpServer, Responder};
mod repository;
mod utils;
mod error;
mod prelude;
use repository::surrealdb_repo::SurrealDBRepo;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init().await.expect("Error connecting to SurrealDB!");
    let db_data = Data::new(surreal);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
