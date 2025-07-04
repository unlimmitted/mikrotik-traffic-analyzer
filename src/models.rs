use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::traffic)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Traffic {
    pub name: String,
    pub rx: String,
    pub tx: String,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub name: String,
    pub rx: String,
    pub tx: String,
}