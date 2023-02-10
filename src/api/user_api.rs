use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::repository::surrealdb_repo::SurrealDBRepo;
use crate::model::user_model::{NewUser, UserBMC, UserEntry};


#[post("/user")]
pub async fn create_user(db: Data<SurrealDBRepo>, new_user: Json<NewUser>) -> HttpResponse {
    let data = UserEntry {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned()
    };
    
    match UserBMC::create(db, "user", data ).await{
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    
    let todo_detail = UserBMC::get(db, &id).await;
    
    match todo_detail {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_users (db: Data<SurrealDBRepo>) -> HttpResponse {
    let result = UserBMC::get_all(db).await;
    
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
   }
}

// pub async fn create_user(db: Data<SurrealDBRepo>, new_user: Json<User>) -> HttpResponse{
//     if let User::Create { name, email, password } = new_user.0 {
    
//     }
//     todo!()
// }
// #[get("/user/login/{cookie}")]
// pub async fn login_user(db: Data<SurrealDBRepo>, login: Json<User>) -> HttpResponse{
//     if let User::LogIn{name, password} = login.0 {
        
//     }
//     todo!()
// }

// pub async fn delete_user() -> HttpResponse{

// }