use crate::migration;
use crate::models::Traffic;
use crate::schema::traffic;
use crate::schema::traffic::dsl::traffic as other_traffic;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct DbConnector {
    pub connection: SqliteConnection,
}

impl DbConnector {
    pub fn new() -> Self {
        let database_url = "./src/database/db.sqlite";
        let mut conn = SqliteConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        migration::run_migrations(&mut conn);
        Self { connection: conn }
    }

    pub async fn get_traffic(&mut self) -> Vec<Traffic> {
        let all_traffic: Vec<Traffic> = other_traffic
            .load(&mut self.connection)
            .expect("Ошибка при загрузке пользователей");
        all_traffic
    }

    pub fn insert_or_update_traffic(&mut self, data: Traffic) {
        diesel::insert_into(traffic::table)
            .values(&data)
            .on_conflict(traffic::name)
            .do_update()
            .set((traffic::tx.eq(&data.tx), traffic::rx.eq(&data.rx)))
            .execute(&mut self.connection)
            .expect("Ошибка вставки/обновления в traffic");
    }
}
