use actix_web::{
    post, get, put, delete,
    web::{Data, Json, Path},
    HttpResponse, cookie::time::Error,
};
use courier::account_data::AccountData;
use surrealdb::sql::Value;
use crate::{repository::surrealdb_repo::SurrealDBRepo, model::user_model::{get_user_by_email, update_user}, utils::macros::map};


#[post("/user/create")]
pub async fn create_user(db: Data<SurrealDBRepo>, user: Json<AccountData>) -> HttpResponse {
    let email = if let Some(email) =  &user.email {email} else {
        return HttpResponse::BadRequest().body("email adress allready taken"); // TODO unsucure
    };
    //if we have a user name match do not make new user
    match get_user_by_email(&db, email).await {
        Ok(object) => {
            match object.get("email"){
                Some(_) =>  HttpResponse::Ok().json(false),
                None => {
                    let data = AccountData{
                        name: user.name.clone(),
                        email: user.email.clone(),
                        password: user.password.clone(),
                        ..Default::default()
                    };
                    if let Err(err) = db.create("user", data).await{
                        HttpResponse::InternalServerError().body(err.to_string())
                    } else {
                        HttpResponse::Ok().json(true)
                    }
                }
            }

        },  
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/user/login")]
pub async fn login(db: Data<SurrealDBRepo>, user: Json<AccountData>) -> HttpResponse{
    if user.loggedin{
        return HttpResponse::Ok().json("already logged in");
    }
    let user_email = if let Some(email) =  &user.email {email} else {
        return HttpResponse::BadRequest().body("email adress allready taken"); // TODO unsucure
    };
    let user_password = if let Some(password) =  &user.password {password} else {
        return HttpResponse::BadRequest().body("password allready taken"); // TODO unsucure
    };
    match get_user_by_email(&db, user_email).await {
        Ok(mut object) => {
            let password = if let Some(password) = object.get("password"){
                password
            } else {
                return HttpResponse::Ok().json(false);
            };
            
            let email = if let Some(email) = object.get("email"){
                email
            } else {
                return HttpResponse::Ok().json(false);
            };
            
            let user_password: Value = user_password.clone().into(); 
            let user_email: Value = user_email.clone().into();

            if user_password != *password{
                return  HttpResponse::Ok().json("bad password");
            }
            if user_email != *email{
                return  HttpResponse::Ok().json("bad email");

            }

            if let Some(logged_in) = object.get_mut("loggedin"){
                *logged_in = Value::from(true);
            }

            if let Some(table_id) = object.get("id"){
                let r = update_user(db, &table_id.clone().as_string(), object.clone()).await;
                let object = if let Ok(object) =r{
                    object
                } else {
                    object
                };

                HttpResponse::Ok().json(object)

            } else {
                HttpResponse::Ok().json(object)
            }

        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[put("/user/logout")]
pub async fn logout(db: Data<SurrealDBRepo>, user: Json<AccountData>) -> HttpResponse{
    todo!()
}

#[get("/user/data/{email}")]
pub async fn get_user_data(db: Data<SurrealDBRepo>, user: Json<AccountData>) -> HttpResponse{



    todo!()
}
