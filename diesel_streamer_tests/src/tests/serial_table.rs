#[cfg(test)]
mod tests {
    use crate::counter::Counter;
    use crate::factory::{self, User};
    use crate::test_runner;

    #[test]
    fn allows_processing_table_data() {
        test_runner::run_test(|conn| {
            factory::insert_users(2, conn);

            let all_users = factory::get_users(conn);

            use factory::users::dsl::{id, users};

            diesel_streamer::stream_serial_table!(users, id, conn, |loaded_users: Vec<User>| {
                assert_eq!(loaded_users.first(), all_users.first());
                assert_eq!(loaded_users.last(), all_users.last())
            });
        });
    }

    #[test]
    fn does_nothing_when_table_is_empty() {
        test_runner::run_test(|conn| {
            use factory::users::dsl::{id, users};

            let call_count = Counter::new(0);

            diesel_streamer::stream_serial_table!(users, id, conn, |_loaded_users: Vec<User>| {
                call_count.increment()
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

            let beginning = user_with_least_id.id + 1;
            let chunk_size = 200;

            diesel_streamer::stream_serial_table!(
                users,
                id,
                conn,
                chunk_size,
                Some(beginning),
                move |loaded_users: Vec<User>| {
                    assert_eq!(loaded_users.len(), ((inserted_users_count - 1) as usize));

                    assert!(!loaded_users.contains(user_with_least_id));
                }
            );
        });
    }

    #[test]
    fn stops_at_specified_end() {
        assert_eq!(1, 2);
    }

    #[test]
    fn stops_at_the_max_end_when_no_end_is_specified() {
        assert_eq!(1, 2);
    }
}
