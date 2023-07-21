use std::env;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenvy::dotenv;

use crate::db;

pub fn run_test<TestFn>(test_fn: TestFn)
where
    TestFn: FnOnce(&mut PgConnection),
{
    let mut conn = match should_skip_db_setup() {
        true => db::establish_connection(),
        false => db::setup(),
    };

    conn.test_transaction(|conn| -> Result<(), ()> {
        test_fn(conn);

        Ok(())
    });
}

fn should_skip_db_setup() -> bool {
    dotenv().ok();

    env::var("SKIP_DB_SETUP").is_ok()
}
