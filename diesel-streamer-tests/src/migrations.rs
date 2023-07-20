use diesel::{pg::PgConnection, sql_query, RunQueryDsl};

pub fn run(conn: &mut PgConnection) {
    create_user_table(conn);
}

fn create_user_table(conn: &mut PgConnection) {
    sql_query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
    "#,
    )
    .execute(conn)
    .unwrap();
}
