use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use actix_web::web::Data;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value, thing, Array};

use crate::prelude::*;
use crate::utils::{macros::map};
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};

#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    name: String,
    psw: u64
}

impl User{
    pub fn new(name: String, password: &str) -> Self{
        let mut hasher = DefaultHasher::new();
        password.hash(&mut hasher);
        let psw = hasher.finish();
        Self { name, psw }
    }
}

impl From<User> for Value {
    fn from(value: User) -> Self {
        map!{
            "name".into() => value.name.into(),
            "psw".into() => value.psw.into()
        }.into()
    }
}