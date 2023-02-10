use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::repository::surrealdb_repo::SurrealDBRepo;
use crate::model::{user_model::{NewUser, UserEntry, LoginUser}, user_model};


#[post("/user")]
pub async fn create_user(db: Data<SurrealDBRepo>, new_user: Json<NewUser>) -> HttpResponse {
    //making the new entry
    let data = UserEntry {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned()
    };
    match user_model::create_user(db, data.clone()).await {
        Ok(b) => {
            if b {
                HttpResponse::Ok().json("new user")
            } else {
                HttpResponse::Ok().json("already existed")
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn login(db: Data<SurrealDBRepo>, new_login: Json<LoginUser>) -> HttpResponse{


    todo!()
}

// #[get("/users")]
// pub async fn get_users (db: Data<SurrealDBRepo>) -> HttpResponse {
//     let result = UserBMC::get_all(db).await;
    
//     match result {
//         Ok(todos) => HttpResponse::Ok().json(todos),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//    }
// }
// #[get("/user/{id}")]
// pub async fn get_user(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
    
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }
    
//     let todo_detail = db.get(&id, "user", None).await.unwrap().into_iter().next().unwrap();
    
//     match todo_detail {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }