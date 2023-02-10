use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::repository::surrealdb_repo::SurrealDBRepo;
use crate::model::user_model::{User};

pub async fn create_user(db: Data<SurrealDBRepo>, new_user: Json<User>) -> HttpResponse{
   match new_user.0{
        User::Create { name, email, password } => {
            
        },
        _ => {},
   }
    todo!()
}

pub async fn login_user() -> HttpResponse{
    todo!()
}