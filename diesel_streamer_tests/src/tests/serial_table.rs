#[cfg(test)]
mod tests {
    use crate::db;
    use crate::factory::{self, User};
    use std::sync::Mutex;

    #[test]
    fn allows_processing_table_data() {
        db::run_test(|conn| {
            factory::insert_users(1..=2, conn);

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
        db::run_test(|conn| {
            use factory::users::dsl::{id, users};

            let was_called_at_least_once = Mutex::new(false);

            diesel_streamer::stream_serial_table!(users, id, conn, |_loaded_users: Vec<User>| {
                *was_called_at_least_once.lock().unwrap() = true;
            });

            assert!(!was_called_at_least_once.into_inner().unwrap());
        });
    }

    #[test]
    fn allows_processing_table_data_in_chunks() {
        assert_eq!(1, 2);
    }

    #[test]
    fn starts_from_specified_beginning() {
        assert_eq!(1, 2);
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
