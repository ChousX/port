use std::{sync::Arc, fmt::format};
use futures::Future;
use surrealdb::{sql::{Value, Object, thing}, Response};
use std::collections::BTreeMap;
use crate::utils::{macros::map};
use surrealdb::{Datastore, Session};
use crate::prelude::*;

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let ds = Arc::new(Datastore::new("file://surreal.db").await?);
        
        let ses = Session::for_kv().with_ns("test").with_db("test");

        Ok(SurrealDBRepo { ses, ds })
    }

    pub async fn create<T: Into<Value>>(&self, title: &str, data: T) -> Result<(), Error> {
        let sql = "CREATE type::table($title) CONTENT $data";
        let data: Object = W(data.into()).try_into()?;
        let vars: BTreeMap<String, Value> = map!{
            "title" => title,
            "data" => Value::from(data)
        };
        match self.ds.execute(sql, &self.ses, Some(vars), false).await{
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
