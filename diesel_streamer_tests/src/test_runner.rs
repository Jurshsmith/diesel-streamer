use diesel::prelude::*;
use diesel::PgConnection;

use crate::db;

pub fn run_test<TestFn>(test_fn: TestFn)
where
    TestFn: FnOnce(&mut PgConnection),
{
    let mut conn = db::establish_connection();

    conn.test_transaction(|conn| -> Result<(), ()> {
        test_fn(conn);

        Ok(())
    });
}
