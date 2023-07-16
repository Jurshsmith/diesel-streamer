use crate::{db, Benchmarkable};
use diesel::{insert_into, prelude::*, sql_query, sql_types::Integer, Insertable};

pub struct SerialTable;

impl Benchmarkable for SerialTable {
    fn setup() {
        Self::create_table();
        Self::insert_bulk_data();
    }

    fn do_eager_loading() {
        use crate::serial_table::users::dsl::users;

        let mut conn = db::connect();

        let loaded_users = users.load::<User>(&mut conn).unwrap();

        loaded_users.into_iter().for_each(|user| {
            // Process serial table eagerly
            // Self::process_data(&user)
        })
    }

    fn do_lazy_loading() {
        use crate::serial_table::users::dsl::{id, users};

        let mut conn = db::connect();

        diesel_streamer::stream_serial_table!(users, id, conn, |loaded_users: Vec<User>| {
            loaded_users.into_iter().for_each(|user| {
                // Process serial table lazily
                // Self::process_data(&user)
            });
        })
    }
}

impl SerialTable {
    fn create_table() {
        let mut conn = db::connect();

        sql_query("DROP TABLE IF EXISTS users;")
            .execute(&mut conn)
            .unwrap();

        sql_query(
            r#"
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
    "#,
        )
        .execute(&mut conn)
        .unwrap();
    }

    fn process_data(user: &User) {
        let mut conn = db::connect();

        sql_query("UPDATE users SET updated_at = NOW() WHERE id = $1")
            .bind::<Integer, _>(user.id)
            .execute(&mut conn)
            .unwrap();
    }

    // TODO: Make these env variables?
    const TOTAL_BULK_SIZE: i32 = 1000000;
    const MAX_BULK_CHUNK_SIZE: usize = 65535;
    fn insert_bulk_data() {
        use crate::serial_table::users::dsl::users;

        let mut conn = db::connect();

        let unsaved_users = (1..=Self::TOTAL_BULK_SIZE)
            .map(|index| UnsavedUser {
                name: format!("User Name {}", index),
            })
            .collect::<Vec<UnsavedUser>>();

        for unsaved_users in unsaved_users.chunks(Self::MAX_BULK_CHUNK_SIZE) {
            insert_into(users)
                .values(unsaved_users)
                .execute(&mut conn)
                .unwrap();
        }
    }
}

// A Serial Table
table! {
    users (id) {
        id -> Serial,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
struct UnsavedUser {
    pub name: String,
}

#[derive(Queryable)]
#[allow(dead_code)]
struct User {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
