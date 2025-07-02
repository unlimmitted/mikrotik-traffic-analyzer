use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::traffic)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Traffic {
    pub name: String,
    pub tx: String,
    pub rx: String,
}