#[cfg(test)]
mod tests {
    use crate::counter::Counter;
    use crate::factory::{self, User};
    use crate::test_runner;

    #[test]
    fn allows_processing_table_data() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            factory::insert_users(2, conn);

            let all_users = factory::get_users(conn);

            diesel_streamer::stream_serial_table!(users, id, conn, |loaded_users: Vec<User>| {
                assert_eq!(loaded_users.first(), all_users.first());
                assert_eq!(loaded_users.last(), all_users.last());
            });
        });
    }

    #[test]
    fn allows_processing_tables_with_one_row() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            factory::insert_users(1, conn);

            let all_users = factory::get_users(conn);

            let mut call_count = Counter::new(0);

            diesel_streamer::stream_serial_table!(users, id, conn, |loaded_users: Vec<User>| {
                call_count.increment();
                assert_eq!(loaded_users.len(), 1);
                assert_eq!(loaded_users.first(), all_users.first());
            });

            assert_eq!(*call_count.value, 1);
        });
    }

    #[test]
    fn does_nothing_when_table_is_empty() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            let mut call_count = Counter::new(0);

            diesel_streamer::stream_serial_table!(users, id, conn, |_loaded_users: Vec<User>| {
                call_count.increment();
            });

            assert_eq!(*call_count.value, 0);
        });
    }

    #[test]
    fn allows_processing_table_data_in_chunks() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            factory::insert_users(2, conn);

            let chunk_size = 1;

            diesel_streamer::stream_serial_table!(users, id, conn, 1, |loaded_users: Vec<User>| {
                assert!(loaded_users.len() <= chunk_size);
            });
        });
    }

    #[test]
    fn starts_from_specified_beginning() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            let inserted_users_count = 4;
            factory::insert_users(inserted_users_count, conn);

            let inserted_users = factory::get_users(conn);
            let user_with_least_id = inserted_users.first().unwrap();

            let beginning = Some(user_with_least_id.id + 1);
            let chunk_size = 200;

            diesel_streamer::stream_serial_table!(
                users,
                id,
                conn,
                chunk_size,
                beginning,
                move |loaded_users: Vec<User>| {
                    assert_eq!(loaded_users.len(), ((inserted_users_count - 1) as usize));

                    assert!(!loaded_users.contains(user_with_least_id));
                }
            );
        });
    }

    #[test]
    fn stops_at_specified_end() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            let inserted_users_count = 4;
            factory::insert_users(inserted_users_count, conn);

            let inserted_users = factory::get_users(conn);
            let user_with_lowest_id = inserted_users.first().unwrap();
            let user_with_highest_id = inserted_users.last().unwrap();

            let beginning = Some(inserted_users.first().unwrap().id);
            let end = Some(user_with_highest_id.id - 1);
            let chunk_size = user_with_highest_id.id - user_with_lowest_id.id;

            diesel_streamer::stream_serial_table!(
                users,
                id,
                conn,
                chunk_size,
                beginning,
                end,
                move |loaded_users: Vec<User>| {
                    assert_eq!(loaded_users.len(), ((inserted_users_count - 1) as usize));

                    assert!(!loaded_users.contains(user_with_highest_id));
                }
            );
        });
    }
}
