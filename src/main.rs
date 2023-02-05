use actix_web::{get, web::Data, App, HttpServer, Responder};
mod repository;
mod utils;
mod error;
mod prelude;
pub mod model;
mod api;
use repository::surrealdb_repo::SurrealDBRepo;
use api::todo_api::{create_todo, get_todos, get_todo, update_todo, delete_todo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init().await.expect("Error connecting to SurrealDB!");
    let db_data = Data::new(surreal);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_todo) 
            .service(get_todos) 
            .service(get_todo) 
            .service(update_todo) 
            .service(delete_todo) 

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
