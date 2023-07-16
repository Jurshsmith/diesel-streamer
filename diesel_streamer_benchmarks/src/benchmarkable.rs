use sysinfo::{System, SystemExt};

pub trait Benchmarkable {
    fn setup() {}

    fn do_eager();
    fn do_lazy();

    /// Uses a naive benchmarking strategy
    /// Basically records the memory delta for each approach
    fn benchmark() {
        Self::setup();

        let mut sys = System::new_all();

        sys.refresh_all();

        println!("Initial Memory Stats:");
        println!("used memory : {} bytes", sys.used_memory());

        Self::do_eager();

        println!("After doing Eager Memory Stats:");
        println!("used memory : {} bytes", sys.used_memory());

        Self::do_lazy();

        println!("After doing Lazy Memory Stats:");
        println!("used memory : {} bytes", sys.used_memory());
    }
}
