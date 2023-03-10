//9:24
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

pub struct LoginRequest{
    user_name: String,
    password: String,
}

