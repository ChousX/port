use actix_web::{App, HttpServer, middleware::Logger, web::Data};

mod api;
mod model;
mod repository;
mod utils;
mod prelude;
// mod error;
mod account;

use api::task::{get_task

};

use repository::surrealdb_repo::SurrealDBRepo; // Add
use api::todo_api::{create_todo, get_todos, get_todo, update_todo, delete_todo}; // Add

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    let surreal = SurrealDBRepo::init().await.expect("Error connecting to SurrealDB!"); // Add
    
    let db_data = Data::new(surreal); // Add

    HttpServer::new( move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(db_data.clone()) 
            .service(create_todo) // Add
            .service(get_todos) // Add
            .service(get_todo) // Add
            .service(update_todo) // Add
            .service(delete_todo) 
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}