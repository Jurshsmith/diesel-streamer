use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (db_name, db_raw_url) = get_db_name_and_raw_url(&database_url);

    let mut conn = PgConnection::establish(&db_raw_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_raw_url));

    drop_database(&db_name, &mut conn);
    create_database(&db_name, &mut conn);

    conn
}

fn get_db_name_and_raw_url(url: &str) -> (String, String) {
    let mut url_split = url.split('/').collect::<Vec<&str>>();

    let db_name = url_split
        .pop()
        .expect("DATABAE NAME needs to be specified. See: sample.env");
    let db_raw_url = url_split.join("/");

    (db_name.to_string(), db_raw_url)
}

fn drop_database(db_name: &str, conn: &mut PgConnection) {
    diesel::sql_query(format!(r#"DROP DATABASE IF EXISTS "{}""#, db_name))
        .execute(conn)
        .unwrap();
}

fn create_database(db_name: &str, conn: &mut PgConnection) {
    diesel::sql_query(format!(r#"CREATE DATABASE "{}""#, db_name))
        .execute(conn)
        .unwrap();
}
