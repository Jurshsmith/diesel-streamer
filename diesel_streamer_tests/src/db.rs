use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::migrations;

pub fn run_test<TestFn>(test_fn: TestFn)
where
    TestFn: FnOnce(&mut PgConnection),
{
    let db_url = database_url();

    PgConnection::establish(&db_url).unwrap_or_else(|_error| {
        let (db_name, db_raw_url) = get_db_name_and_raw_url(&db_url);

        let mut raw_conn = connect_to_database_url_or_panic(&db_raw_url);

        create_database(&db_name, &mut raw_conn);

        let mut conn = connect();

        migrations::run(&mut conn);

        conn.test_transaction(|conn| -> Result<(), ()> {
            test_fn(conn);

            Ok(())
        });

        conn
    });
}

fn connect() -> PgConnection {
    connect_to_database_url_or_panic(&database_url())
}

fn database_url() -> String {
    dotenv().ok();

    env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL env variable needs to be set.")
}

fn get_db_name_and_raw_url(url: &str) -> (String, String) {
    let mut url_split = url.split('/').collect::<Vec<&str>>();

    let db_name = url_split
        .pop()
        .expect("DATABAE NAME needs to be specified. See: sample.env");
    let db_raw_url = url_split.join("/");

    (db_name.to_string(), db_raw_url)
}

fn create_database(db_name: &str, conn: &mut PgConnection) {
    diesel::sql_query(format!(r#"CREATE DATABASE "{}""#, db_name))
        .execute(conn)
        .unwrap();
}

fn connect_to_database_url_or_panic(db_url: &str) -> PgConnection {
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
