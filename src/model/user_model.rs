use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use actix_web::web::{Data, Form};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value, thing, Array};

use crate::prelude::*;
use crate::utils::{macros::map};
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use rand::thread_rng;
use pwbox::{Eraser, ErasedPwBox, Suite, sodium::Sodium};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser{
    pub name: String,
    pub email: String,
    pub password: String,
}
impl Patchable for NewUser {}

impl From<NewUser> for Value{
    fn from(value: NewUser) -> Self {
        map!{
            "name".into() => value.name.into(),
            "email".into() => value.email.into(),
            "password".into() => value.password.into()
        }.into()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntry{
    pub id: Option<String>,
    pub name: String, 
    pub email: String,
    pub password: String,
}

impl From<UserEntry> for Value {
    fn from(value: UserEntry) -> Self {
        match value.id{
            Some(id) => {
                map!{
                    "id".into() => id.into(),
                    "name".into() => value.name.into(),
                    "email".into() => value.email.into(),
                    "password".into() => value.password.into(),
                }.into()
            },
            None => {
                map!{
                    "name".into() => value.name.into(),
                    "email".into() => value.email.into(),
                    "password".into() => value.password.into(),
                }.into()
            }
        }
    }
}

impl Creatable for UserEntry {}

pub struct UserBMC{}
impl UserBMC{
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM user;";

        let res = db.ds.execute(ast, &db.ses, None, true).await?;
        
        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn create<T: Creatable>(db: Data<SurrealDBRepo>, tb: &str, data: T) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";
        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
            "tb".into() => tb.into(),
            "data".into() => Value::from(data)
        ];

        let responses = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
        let first_val = responses.into_iter().next().map(|r| r.result).expect("id not returned")?;

        W(first_val.first()).try_into()
    }
    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";
            
            let tid = format!("todo:{}", tid);

            let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
    
            let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
    
            let first_res = ress.into_iter().next().expect("Did not get a response");
    
            W(first_res.result?.first()).try_into()
           
    }
    pub async fn update<T: Patchable>(db: Data<SurrealDBRepo>, tid: &str, data: T) -> Result<Object, Error> {
    	let sql = "UPDATE $th MERGE $data RETURN *";

        let tid = format!("todo:{}", tid);

    	let vars = map![
	    "th".into() => thing(&tid)?.into(),
	    "data".into() => data.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        let result = first_res.result?;
        
        W(result.first()).try_into()
    }
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
