//9:24 - 9:33
use actix_web::{
    get,
    post,
    put,

    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{
        header::ContentType,
        StatusCode
    }
};

use serde::{
    Serialize, Deserialize
};

use derive_more::Display;

#[get("/api/create_account/{task_global_id}")]
pub struct CreationRequest{
    user_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Display)]
pub struct TaskIdentifier {
    task_global_id: String
}