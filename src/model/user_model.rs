use actix_web::web::Data;
use actix_web_lab::__reexports::tracing::Value;
use courier::account_data::AccountData;
use surrealdb::sql::{Object, thing};

use crate::repository::surrealdb_repo::SurrealDBRepo;
use crate::prelude::*;
use crate::utils::macros::map;


// emails should be unquick
pub async fn get_user_by_email(db: &Data<SurrealDBRepo>, email: &str) -> Result<Object, Error> {
    let sql = format!("SELECT * FROM user WHERE email CONTAINS '{}'", email);
    let ress = db.ds.execute(&sql, &db.ses, None, false).await?;
    let first = ress.into_iter().next().expect("did not get a response");
    W(first.result?.first()).try_into()
}

pub async fn update_user(db: Data<SurrealDBRepo>, table_id: &str, account_data: Object) -> Result<Object, Error>{
    let sql = "UPDATE $table_id MERGE $data RETURN *";
    let table_id = format!("user:{}", table_id);

    let vars = map!{
        "table_id" => thing(&table_id)?,
        "account_data" => account_data
    };

    let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
    let first_res = ress.into_iter().next().expect("id not returned");
    let result = first_res.result?;
    W(result.first()).try_into()
}







// use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
// use actix_web::web::{Data, Form};
// use std::collections::BTreeMap;
// use serde::{Serialize, Deserialize};
// use surrealdb::sql::{Object, Value, thing, Array};

// use courier::account_data::AccountData;

// use crate::prelude::*;
// use crate::utils::{macros::map};
// use crate::repository::surrealdb_repo::{SurrealDBRepo};
// use rand::thread_rng;
// use pwbox::{Eraser, ErasedPwBox, Suite, sodium::Sodium};


// pub async fn create_user(db: Data<SurrealDBRepo>, data: AccountData) -> Result<bool, Error>{
//     let sql = format!("SELECT * FROM user WHERE email CONTAINS '{}' LIMIT 1", data.name.unwrap()); 
//     //there has to be a better way....
//     let res =  db.ds.execute(&sql, &db.ses, None, true).await?;
//     let r = res.into_iter().next().unwrap();
//     let array: Array = W(r.result?).try_into()?;
//     let array: Result<Vec<Object>, Error> = array.into_iter().map(|value| W(value).try_into()).collect();
//     let array = array?;
//     if array.is_empty(){
//         db.create("user", data).await?;
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }

// pub async fn login_user(db: Data<SurrealDBRepo>, data: AccountData) -> Result<Object, Error>{
//     let sql = format!("SELECT password FROM user WHERE name CONTAINS '{}' LIMIT 1", data.name);
//     let res =  db.ds.execute(&sql, &db.ses, None, true).await?;
//     let first_res = res.into_iter().next().expect("id not returned");
//     let result = first_res.result?;
//     todo!()
// }

// // pub struct UserBMC{}
// // impl UserBMC{
// //     pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
// //         let ast = "SELECT * FROM user;";

// //         let res = db.ds.execute(ast, &db.ses, None, true).await?;
        
// //         let first_res = res.into_iter().next().expect("Did not get a response");

// //         let array: Array = W(first_res.result?).try_into()?;

// //         array.into_iter().map(|value| W(value).try_into()).collect()
// //     }

// //     pub async fn create<T: Creatable>(db: Data<SurrealDBRepo>, tb: &str, data: T) -> Result<Object, Error> {
// //         let sql = "CREATE type::table($tb) CONTENT $data RETURN *";
// //         let data: Object = W(data.into()).try_into()?;

// //         let vars: BTreeMap<String, Value> = map![
// //             "tb".into() => tb.into(),
// //             "data".into() => Value::from(data)
// //         ];

// //         let responses = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
// //         let first_val = responses.into_iter().next().map(|r| r.result).expect("id not returned")?;

// //         W(first_val.first()).try_into()
// //     }
// //     pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
// //         let sql = "SELECT * FROM $th";
            
// //             let tid = format!("user:{}", tid);

// //             let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
    
// //             let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
    
// //             let first_res = ress.into_iter().next().expect("Did not get a response");
    
// //             W(first_res.result?.first()).try_into()
           
// //     }
// //     pub async fn update<T: Patchable>(db: Data<SurrealDBRepo>, tid: &str, data: T) -> Result<Object, Error> {
// //     	let sql = "UPDATE $th MERGE $data RETURN *";

// //         let tid = format!("todo:{}", tid);

// //     	let vars = map![
// // 	    "th".into() => thing(&tid)?.into(),
// // 	    "data".into() => data.into()];

// //         let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

// //         let first_res = ress.into_iter().next().expect("id not returned");

// //         let result = first_res.result?;
        
// //         W(result.first()).try_into()
// //     }
// // }
