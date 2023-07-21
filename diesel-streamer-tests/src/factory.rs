use diesel::{pg::PgConnection, prelude::*, Insertable};

// User Factory

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
pub struct UnsavedUser {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable)]
#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub fn insert_users(number_of_users: u16, conn: &mut PgConnection) {
    use self::users::dsl::users;

    let unsaved_users: Vec<UnsavedUser> = (1..=number_of_users)
        .map(|index| UnsavedUser {
            name: format!("UserName {}", index),
        })
        .collect();

    diesel::insert_into(users)
        .values(unsaved_users)
        .execute(conn)
        .unwrap();
}

pub fn insert_user(name: &str, conn: &mut PgConnection) -> User {
    use self::users::dsl::users;

    diesel::insert_into(users)
        .values(UnsavedUser {
            name: name.to_string(),
        })
        .get_result(conn)
        .unwrap()
}

pub fn get_users(conn: &mut PgConnection) -> Vec<User> {
    use self::users::dsl::{id, users};

    users.order_by(id).get_results::<User>(conn).unwrap()
}

pub fn get_user_by_name(user_name: &str, conn: &mut PgConnection) -> Option<User> {
    use self::users::dsl::{name, users};

    users
        .filter(name.eq(user_name.to_string()))
        .first::<User>(conn)
        .optional()
        .unwrap()
}
