use actix_web::{web::Data, App, HttpServer, web::scope};
mod repository;
mod utils;
mod error;
mod prelude;
pub mod model;
mod api;
use repository::surrealdb_repo::SurrealDBRepo;
use api::user_api::*;
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
                    .service(create_user) 
                    // .service(get_user) 
                    // .service(get_users)
            )
            // .service(
            //     spa()
            //         .index_file("./dist/index.html")
            //         .static_resources_mount("/")
            //         .static_resources_location("./dist")
            //         .finish()
            // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
