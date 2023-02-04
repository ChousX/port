use actix_web::web::Data;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value, thing, Array};

use crate::prelude::*;
use crate::utils::{macros::map};
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub body: String,
}

impl From<Todo> for Value {
    fn from(val: Todo) -> Self {
        match val.id {
            Some(v) => {
                map![
                    "id".into() => v.into(),
                    "title".into() => val.title.into(),
                    "body".into() => val.body.into(),
            ].into()
            },
            None => {
                map![
                    "title".into() => val.title.into(),
                    "body".into() => val.body.into()
                ].into()
            }
        }
    }
}

impl Creatable for Todo{}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub body: Option<String>,
}

impl From<TodoPatch> for Value {
    fn from(val: TodoPatch) -> Self {

        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        
        if let Some(t) = val.title {
            value.insert("title".into(), t.into());
        }

        if let Some(b) = val.body {
            value.insert("body".into(), b.into());
        }
        Value::from(value)
    }
}

impl Patchable for TodoPatch {}
