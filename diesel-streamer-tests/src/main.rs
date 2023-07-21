use diesel_streamer_tests::db;

fn main() {
    // Run once to setup database
    // Useful in a CI environment running parallel tests
    db::setup();
}
