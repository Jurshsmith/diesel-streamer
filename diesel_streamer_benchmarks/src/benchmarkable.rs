use std::io::Bytes;

use sysinfo::{System, SystemExt};

pub trait Benchmarkable {
    fn setup() {}

    fn do_eager_loading();
    fn do_lazy_loading();

    /// Uses a naive benchmarking strategy
    /// Basically records the memory delta for each approach
    fn benchmark() {
        Self::setup();

        let mut sys = System::new_all();

        sys.refresh_all();

        let initial_used_memory = sys.used_memory();

        Self::do_eager_loading();

        sys.refresh_all();

        let eager_loading_used_memory = sys.used_memory() - initial_used_memory;

        println!(
            "Eager Loading used: {} MB",
            bytes_to_megabytes(eager_loading_used_memory)
        );

        Self::do_lazy_loading();

        sys.refresh_all();

        let after_lazy_loading_used_memory = sys.used_memory() - eager_loading_used_memory;

        println!(
            "Lazy Loading with DieselStreamer used: {} MB",
            bytes_to_megabytes(after_lazy_loading_used_memory)
        );
    }
}

fn bytes_to_megabytes(bytes: u64) -> f32 {
    (bytes / 10_u64.pow(6)) as f32
}
