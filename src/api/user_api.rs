use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

// use crate::repository::surrealdb_repo::SurrealDBRepo;
// use crate::model::user_model::{User};


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