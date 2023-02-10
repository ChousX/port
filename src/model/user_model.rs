use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use actix_web::web::Data;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value, thing, Array};

use crate::prelude::*;
use crate::utils::{macros::map};
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};

pub struct UserEntry{
    name: String, 
    email: String,
    password: String,
    date_created: ()
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum User{
//     Create{
//         name: String,
//         email: String,
//         password: String
//     },
//     LogIn{
//         name: String,
//         password: String
//     }
// }

// impl From<User> for Value{
//     fn from(val: User) -> Self {
//         match val{
//             User::Create { name, email, password } => {
//                 map!{
//                     "name".into() => name.into(),
//                     "enail".into() => email.into(),
//                     "password".into() => password.into(),
//                 }.into()
//             },
//             User::LogIn { name, password } => {
//                 map!{
//                     "name".into() => name.into(),
//                     "password".into() => password.into(),
//                 }.into()
//             }
//         }
//     }
// }

// impl Creatable for User {}

// pub struct  UserBMC;

// impl UserBMC {
//     pub async fn create<T: Creatable>(db: Data<SurrealDBRepo>, tb: &str, data: T) -> Result<Object, Error>{
//         let sql = "CREATE type::table($tb) CONTENT $data RETURN *";
//         let data: Object = W(data.into()).try_into()?;
//         let vars: BTreeMap<String, Value> = map!{
//             "tb".into() => tb.into(),
//             "data".into() => Value::from(data)
//         };

//         let response = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
//         let first_val =  response.into_iter().next().map(|r| r.result).expect("id not returned")?;

//         W(first_val.first()).try_into()
//     }
// }
