use actix_web::{get, web::Data, App, HttpServer, Responder, web::scope};
mod repository;
mod utils;
mod error;
mod prelude;
pub mod model;
mod api;
use repository::surrealdb_repo::SurrealDBRepo;
use api::todo_api::{create_todo, get_todos, get_todo, update_todo, delete_todo};
use actix_web_lab::web::spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init().await.expect("Error connecting to SurrealDB!");
    let db_data = Data::new(surreal);

    HttpServer::new(move || {
        App::new()
            .service(
                scope("/api")
                    .app_data(db_data.clone())
                    .service(create_todo) 
                    .service(get_todos) 
                    .service(get_todo) 
                    .service(update_todo) 
                    .service(delete_todo) 
            )
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish()
            )
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
