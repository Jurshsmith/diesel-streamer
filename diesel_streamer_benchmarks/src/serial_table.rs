use crate::{db, Benchmarkable};
use diesel::{insert_into, prelude::*, sql_query, Insertable};

pub struct SerialTable;

impl Benchmarkable for SerialTable {
    fn setup() {
        Self::create_users_table();
        Self::insert_bulk_users();
    }

    fn do_eager() {
        print!("THIS IS SOME VERY LONG LINE.THIS IS SOME VERY LONG LINE.THIS IS SOME VERY LONG LINE.THIS IS SOME VERY LONG LINE.");
    }

    fn do_lazy() {
        print!("THIS IS SOME VERY LONG LINE.");
        print!("THIS IS SOME VERY LONG LINE.");
        print!("THIS IS SOME VERY LONG LINE.");
        print!("THIS IS SOME VERY LONG LINE.");
    }
}

impl SerialTable {
    fn create_users_table() {
        let mut conn = db::establish_connection();

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

    const BULK_USERS_SIZE: i32 = 10000;
    fn insert_bulk_users() {
        use crate::serial_table::users::dsl::users;

        let mut conn = db::establish_connection();

        let unsaved_users: Vec<UnsavedUser> = (1..=Self::BULK_USERS_SIZE)
            .into_iter()
            .map(|index| UnsavedUser {
                name: format!("User Name {}", index),
            })
            .collect();

        insert_into(users)
            .values(&unsaved_users)
            .execute(&mut conn)
            .unwrap();
    }
}

table! {
    users (id) {
        id -> Serial,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
struct UnsavedUser {
    pub name: String,
}

#[derive(Debug, Queryable)]
struct User {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
