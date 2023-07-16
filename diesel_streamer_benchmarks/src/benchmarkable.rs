use sysinfo::{System, SystemExt};

pub trait Benchmarkable {
    fn do_eager();
    fn do_lazy();

    /// Uses a naive benchmarking strategy
    /// Basically records the memory delta for each approach
    fn benchmark() {
        let mut sys = System::new_all();

        sys.refresh_all();

        println!("Initial Memory Stats:");
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());

        Self::do_eager();

        println!("After doing Eager Memory Stats:");
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());

        Self::do_lazy();
        println!("After doing Lazy Memory Stats:");
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
    }
}
